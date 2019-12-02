use crate::ast;
use crate::ctx::*;
use crate::error;
use crate::ir;
use crate::refs::*;
use crate::storage::*;

use std::collections::{HashSet, VecDeque};

pub fn verify_file_path(path: &std::path::Path) -> error::Result<String> {
    let file_ext = path.extension().and_then(std::ffi::OsStr::to_str);
    let file_stem = path.file_stem().and_then(std::ffi::OsStr::to_str);

    if file_ext != Some("fri") {
        let path_str = path.to_str().unwrap();
        return Err(error::FridayError::InvalidFilename(path_str.to_owned()))?;
    }

    Ok(file_stem.unwrap().to_owned())
}

pub fn process_file(ctx: &Context<'_>, file_name: &str) -> error::Result<ModlRef> {
    use crate::ast::OwnedToken;
    use crate::parser::SequenceParser;
    use std::fs;
    use std::path::Path;

    let parse_sequence = SequenceParser::new();
    let file_path = Path::new(file_name);

    let modl_name = verify_file_path(file_path)?;

    let file_text = fs::read_to_string(file_name)?;

    let decls = match parse_sequence.parse(ctx.arena, &file_text) {
        Err(e) => Err(e.map_token(OwnedToken::from))?,
        Ok(decls) => decls,
    };

    let decls = ctx.arena.alloc_slice_copy(&decls);

    let modl_ref = {
        let mut refs = ctx.refs.borrow_mut();
        let mut ast = ctx.ast.borrow_mut();

        let modl_ref = refs.modl.make_ref();
        ast.modl.set(modl_ref, ast::Modl::ModExp(decls));
        modl_ref
    };

    {
        let mut ir = ctx.ir.borrow_mut();
        let mut names = ctx.names.borrow_mut();

        let arena_modl_name = ctx.arena.alloc_slice_copy(modl_name.as_bytes());
        let arena_modl_name = unsafe { std::str::from_utf8_unchecked(arena_modl_name) };

        let global_ir = ir
            .modl
            .get_mut(ctx.global_modl())
            .unwrap()
            .as_record_mut()?;
        global_ir
            .children
            .insert(names.make_ident(arena_modl_name), modl_ref);
    }

    let mut modules = VecDeque::new();
    modules.push_back(DeferredModl {
        modl_ref,
        name: modl_name,
        parent: ctx.global_modl(),
    });

    // Using a VecDeque here changes this to a more
    // natural fill order in ir storage.
    while let Some(deferred) = modules.pop_front() {
        modules.extend(process_modl_ast(&ctx, deferred)?);
    }

    Ok(modl_ref)
}

pub fn process_aliases<'ctx>(ctx: &'ctx Context<'ctx>) -> error::Result<()> {
    let mut aliases = Vec::new();

    for (modl_ref, modl_ast) in &ctx.ast.borrow().modl {
        // Ignore module records, we're just handling aliases here.
        match modl_ast {
            ast::Modl::ModExp(..) => continue,
            _ => aliases.push(modl_ref),
        };
    }

    /*
        First, find another module (whether it's an alias or a record)
        which this name corresponds to by traversing the
        tree of modules and their children (and outer scopes, etc.)
    */
    for &modl_ref in &aliases {
        let aliased_ref = resolve_alias_path(ctx, modl_ref)?;
        let ir = ctx.ir.borrow();
        println!(
            "--- alias {} -> {:?}",
            ir.modl.get(aliased_ref).unwrap().name(),
            aliased_ref
        );
    }

    /*
        Then, simplify these links so that each alias directly
        refers to the appropriate module expression, so that
        we will not ever have to worry about aliases taking
        several hops to resolve.

        This will also detect more kinds of recursive
        or unresolvable alias formulations.
    */
    for &modl_ref in &aliases {
        let aliased_target_ref = resolve_alias_target(ctx, modl_ref)?;
        let mut ir = ctx.ir.borrow_mut();
        let modl_ir = ir.modl.get_mut(modl_ref).unwrap();
        let alias = modl_ir.as_alias_mut().unwrap();
        alias.aliased = Some(aliased_target_ref);
    }

    Ok(())
}

pub fn resolve_alias_target<'ctx>(
    ctx: &'ctx Context<'ctx>,
    modl_ref: ModlRef,
) -> error::Result<ModlRef> {
    resolve_alias_target_check_loops(ctx, modl_ref, HashSet::new())
}

fn resolve_alias_target_check_loops<'ctx>(
    ctx: &'ctx Context<'ctx>,
    modl_ref: ModlRef,
    mut seen_refs: HashSet<ModlRef>,
) -> error::Result<ModlRef> {
    let ir = ctx.ir.borrow();
    let modl_ir = ir.modl.get(modl_ref).unwrap();
    let alias = match modl_ir.as_alias() {
        Ok(alias) => alias,
        Err(_) => return Ok(modl_ref),
    };

    let aliased_ref = alias.aliased.ok_or_else(|| {
        error::UnresolvableModulePath(format!("Alias {} does not resolve!", ctx.wrap(alias)))
    })?;

    if seen_refs.contains(&aliased_ref) {
        Err(error::UnresolvableModulePath(format!(
            "Alias {} is self referential!",
            ctx.wrap(alias)
        )))?
    } else {
        seen_refs.insert(aliased_ref);
        resolve_alias_target_check_loops(ctx, aliased_ref, seen_refs)
    }
}

fn resolve_alias_path<'ctx>(ctx: &'ctx Context<'ctx>, modl_ref: ModlRef) -> error::Result<ModlRef> {
    let (full_scope, alias) = {
        let ir = ctx.ir.borrow();
        let modl_ir = ir.modl.get(modl_ref).unwrap();
        let alias = modl_ir.as_alias()?;

        let scope_ir = ir.modl.get(alias.scope).unwrap();
        let scope_record = scope_ir.as_record()?;
        let mut full_scope = Vec::new();
        full_scope.push(alias.scope);
        full_scope.extend(scope_record.scope.iter());

        match alias.aliased {
            Some(aliased) => return Ok(aliased),
            None => (full_scope, alias.clone()),
        }
    };

    println!("resolving: {}", ctx.wrap(&alias));

    let first = alias.path[0];
    let mut scope_ref = 'outer: loop {
        let ir = ctx.ir.borrow();
        for &scope_ref in full_scope.iter() {
            let record = ir.modl.get(scope_ref).unwrap().as_record()?;
            println!("looking for {} in {}", ctx.wrap(&alias), &record.name);
            match record.children.get(&first) {
                Some(&child_ref) if child_ref != modl_ref => break 'outer scope_ref,
                _ => continue,
            }
        }
        Err(error::UnresolvableModulePath(format!(
            "No such module in enclosing scope: {}\nWhen resolving path: {}",
            ctx.names.borrow().get(first).unwrap(),
            ctx.wrap(&alias)
        )))?
    };

    for &path_elt in alias.path.iter() {
        loop {
            let ir = ctx.ir.borrow();
            let scope_modl = ir.modl.get(scope_ref).unwrap();
            print!(
                "looking for {} in {}...",
                ctx.names.borrow().get(path_elt).unwrap(),
                scope_modl.name()
            );
            match scope_modl {
                ir::Modl::Record(scope_record) => match scope_record.children.get(&path_elt) {
                    Some(&child_ref) if child_ref != modl_ref => {
                        println!(" found: {:?}", &child_ref);
                        scope_ref = child_ref;
                        break;
                    }
                    Some(_) => Err(error::UnresolvableModulePath(format!(
                        "Alias {} is self-referential: {} = {}",
                        ctx.wrap(&alias),
                        ctx.names.borrow().get(path_elt).unwrap(),
                        &scope_record.name
                    )))?,
                    None => Err(error::UnresolvableModulePath(format!(
                        "No such module: {}.{}\nWhen resolving path: {}",
                        &scope_record.name,
                        ctx.names.borrow().get(path_elt).unwrap(),
                        ctx.wrap(&alias)
                    )))?,
                },
                ir::Modl::Alias(_) => {
                    println!(" {:?} = {} is an alias.", &scope_ref, scope_modl.name());
                    drop(ir);
                    scope_ref = resolve_alias_path(ctx, scope_ref)?;
                }
            }
        }
    }
    let mut ir = ctx.ir.borrow_mut();
    let alias = ir.modl.get_mut(modl_ref).unwrap().as_alias_mut()?;
    alias.aliased = Some(scope_ref);

    Ok(scope_ref)
}

#[derive(Debug, Clone)]
pub struct DeferredModl {
    pub name: String,
    pub parent: ModlRef,
    pub modl_ref: ModlRef,
}

pub fn process_modl_ast(ctx: &Context<'_>, modl: DeferredModl) -> error::Result<Vec<DeferredModl>> {
    let mut deferred = Vec::new();

    let mut anon_modl_counter = 0;

    let mut ir = ctx.ir.borrow_mut();
    let mut ast = ctx.ast.borrow_mut();
    let mut refs = ctx.refs.borrow_mut();

    let DeferredModl {
        name,
        parent,
        modl_ref,
    } = modl;

    let &modl_ast = ast.modl.get(modl_ref).expect("Module has no ast!");
    match modl_ast {
        ast::Modl::Named(modl_path) => {
            println!("--- modl: {}", &name);
            println!("alias: {}", &modl_path);

            let ast::ModlPath { path, absolute } = modl_path;
            let parent = if absolute { ctx.global_modl() } else { parent };
            let modl_ir = ir.modl.set(modl_ref, ir::Modl::new_alias(name, parent));
            let alias = modl_ir.as_alias_mut()?;
            let mut names = ctx.names.borrow_mut();
            alias
                .path
                .extend(path.iter().map(|id| names.make_ident(id.0)));
        }
        ast::Modl::ModExp(decls) => {
            println!("--- modl: {}", &name);
            let modl_ir = ir.modl.set(modl_ref, ir::Modl::new(name.clone()));
            let record = modl_ir.as_record_mut()?;
            // record.scope.push(modl_ref);
            record.scope.push(parent);

            for decl in decls.iter() {
                println!("decl: {}", decl);
                match decl {
                    ast::Decl::Def(sig, _) => {
                        let ir_sig: Vec<_> =
                            sig.iter().map(|sign| sign.from_ast(ctx).forget()).collect();

                        let decl_ref = refs.decl.make_ref();
                        ast.decl.set(decl_ref, *decl);

                        record.symbols.new_decl(decl_ref, ir_sig);
                        record.decls.push(decl_ref);
                    }

                    ast::Decl::Con(sig) => {
                        let ir_sig: Vec<_> =
                            sig.iter().map(|sign| sign.from_ast(ctx).forget()).collect();

                        let cons_ref = refs.cons.make_ref();
                        ast.cons.set(cons_ref, *decl);

                        record.symbols.new_cons(cons_ref, ir_sig);
                        record.cons.push(cons_ref);
                    }

                    ast::Decl::Let(..) => {
                        let let_ref = refs.decl.make_ref();
                        ast.decl.set(let_ref, *decl);
                        record.decls.push(let_ref);
                    }

                    ast::Decl::Mod(id, &ast_modl) => {
                        let mut names = ctx.names.borrow_mut();
                        let child_id = names.make_ident(id.0);
                        let child_modl = refs.modl.make_ref();
                        record.children.insert(child_id, child_modl);
                        ast.modl.set(child_modl, ast_modl);

                        deferred.push(DeferredModl {
                            name: format!("{}.{}", name, id.0.to_owned()),
                            parent: modl_ref,
                            modl_ref: child_modl,
                        });
                    }
                    ast::Decl::Use(&ast_modl) => {
                        let new_modl = refs.modl.make_ref();
                        // record.scope.push(new_modl);

                        ast.modl.set(new_modl, ast_modl);
                        deferred.push(DeferredModl {
                            name: format!("{}.<anon{}>", name, anon_modl_counter),
                            parent: modl_ref,
                            modl_ref: new_modl,
                        });
                        anon_modl_counter += 1;
                    }
                }
            }
        }
    }

    Ok(deferred)
}
