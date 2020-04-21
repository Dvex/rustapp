// #![feature(proc_macro_hygiene)]
extern crate db_middleware;
// use clap::{clap_app, crate_version};
// use maud::html;
// use pulldown_cmark::{html::push_html, Event, Parser};
// use regex::Regex;
use failure::Error;
// use std::path::Path;
use db_middleware::Conn;

/*
struct Record {
  line: usize,
  tx: String
}

fn wrap_html(s:&str, css:Option<&str>) -> String {
  let res = html!{
    (maud::DOCTYPE)
    html {
      head {
        meta charset="utf-8";
        @if let Some(s) = css {
          link rel="stylesheet" type="text/css" href=(s) {}
        }
      }
      body {
        (maud::PreEscaped(s))
      }
    }
  };
  res.into_string()
}

fn process_file<P: AsRef<Path>>(p: P, re: Regex) -> Result<Vec<Record>, String> {
  let mut res = Vec::new();
  let bts = std::fs::read(p).map_err(|e| "Could not read string".to_string())?;
  if let Ok(ss) = String::from_utf8(bts) {
    for (i, l) in ss.lines().enumerate() {
      if re.is_match(l) {
        res.push(Record {
          line: i,
          tx: l.to_string()
        });
      }
    }
  }
  Ok(res)
}
*/

fn main() -> Result<(), Error> {
  let conn = Conn::new()?;
  /*
  let r = conn.put_currency("US", "Dolares from USA")?;
  let r = conn.put_product("Producto 1")?;
  println!("Added currency: {:?}", r);
  */
  /*
  let r = conn.find_currency(Some("PEN".to_owned()), 10)?;
  for p in r {
    println!("\n-----Currency--------\n");
    println!("{:?}", p);
  }
  */

  let r = conn.find_sale(Some(7), Some(1), 2000, 1, "WEB".to_string())?;
  for p in r {
    println!("\n-----SALE--------\n");
    println!("{:?}", p);
  }

  /*
  let r = conn.set_currency(1, "PEN S/.")?;
  println!("Updated currency: {:?}", r);
  */
  
  Ok(())
  /*
  let clap = clap_app!( actix_app =>
                        (version:crate_version!())
                        (author:"Eduardo Cabrera")
                        (about:"Example of how work clap-actix")
                        (@arg input: +required "Sets the input file")
                        (@arg wrap: -w "Wrap in html")
                        (@arg event: -e "Print event")
                        (@arg css: --css +takes_value "Link to css")
                      ).get_matches();
  
  // println!("Input = {:?}", clap.value_of("input"));
  let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read file");
  let ps = Parser::new(&infile);
  
  let ps : Vec<Event> = ps.into_iter().collect();
  if clap.is_present("event") {
    for p in &ps {
      println!("{:?}", p);
    }
  }

  let mut res = String::new();
  push_html(&mut res, ps.into_iter());

  if clap.is_present("wrap") {
    res = wrap_html(&res, clap.value_of("css"));
  }

  println!("{}", res);
  */
}
