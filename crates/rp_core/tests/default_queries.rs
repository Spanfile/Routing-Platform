mod common;

use anyhow::anyhow;
use rp_core::config::Config;
use std::rc::Rc;

#[test]
fn nonexistent_default_cat_query() -> anyhow::Result<()> {
    let schema = Rc::new(common::get_nonexistent_default_cat_query_schema()?);
    let result = Config::from_schema(Rc::downgrade(&schema));

    match result {
        Ok(_) => Err(anyhow!("config creation succeeded")),
        Err(e) => {
            if let Some(std::io::Error { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_default_cat_query() -> anyhow::Result<()> {
    println!("{:?}", std::env::current_dir()?);
    let schema = Rc::new(common::get_invalid_default_cat_query_schema()?);
    let result = Config::from_schema(Rc::downgrade(&schema));

    match result {
        Ok(_) => Err(anyhow!("config creation succeeded")),
        Err(e) => {
            if let Some(rp_core::error::ConstraintError { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn nonexistent_default_ls_query() -> anyhow::Result<()> {
    println!("{:?}", std::env::current_dir()?);
    let schema = Rc::new(common::get_nonexistent_default_ls_query_schema()?);
    let result = Config::from_schema(Rc::downgrade(&schema));

    match result {
        Ok(_) => Err(anyhow!("config creation succeeded")),
        Err(e) => {
            if let Some(std::io::Error { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_default_ls_query() -> anyhow::Result<()> {
    println!("{:?}", std::env::current_dir()?);
    let schema = Rc::new(common::get_invalid_default_ls_query_schema()?);
    let result = Config::from_schema(Rc::downgrade(&schema));

    match result {
        Ok(_) => Err(anyhow!("config creation succeeded")),
        Err(e) => {
            if let Some(rp_core::error::PropertyError::ConstraintNotMet) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
