#![feature(test)]

extern crate piston_meta;
extern crate test;

use test::Bencher;
use piston_meta::*;

#[bench]
fn json(bencher: &mut Bencher) {
    let text = r#"
    {
       "materials": {
            "metal": {
                "reflectivity": 1.0
            },
            "plastic": {
                "reflectivity": 0.5
            }
       },
       "entities": [
            {
                "name": "hero",
                "material": "metal"
            },
            {
                "name": "moster",
                "material": "plastic"
            }
       ]
    }
    "#;

    let rules = r#"
1 "material" [t!"name" w? ":" w? "{" w? "\"reflectivity\"" w? ":" w? $"reflectivity" w? "}"]
2 "materials" ["\"materials\"" w? ":" w? "{" w? s?.(["," w?]){@"material""material"} w? "}"]
3 "entity" ["{" w? s?.(["," w?]){{
    ["\"name\"" w? ":" w? t!"name"]
    ["\"material\"" w? ":" w? t!"material"]
}} w? "}"]
4 "entities" ["\"entities\"" w? ":" w? "[" w? s?.(["," w?]){@"entity""entity"} w? "]"]
5 "document" [w? "{" w? s?(["," w?]){{
    @"materials""materials"
    @"entities""entities"
}} w? "}" w?]
    "#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = bootstrap::convert(
        &parse(&bootstrap::rules(), rules).unwrap(),
        &mut vec![] // stores ignored meta data
    ).unwrap();

    let data = parse(&rules, text);
    match data {
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseErrorHandler::new(&text).error(range, err);
        }
        _ => {}
    }

    bencher.iter(|| {
        let _ = parse(&rules, text).unwrap();
    });
}

#[bench]
fn ron(bencher: &mut Bencher) {
    let text = r#"
    Scene(
        materials: {
            "metal": (
                reflectivity: 1.0,
            ),
            "plastic": (
                reflectivity: 0.5,
            ),
        },
        entities: [
            (
                name: "hero",
                material: "metal",
            ),
            (
                name: "monster",
                material: "plastic",
            ),
        ],
    )
    "#;

    let rules = r#"
1 "material" [t!"name" w? ":" w? "(" w? "reflectivity" w? ":" w? $"reflectivity" ?"," w? ")"]
2 "materials" ["materials" w? ":" w? "{" w? s?.(["," w?]){@"material""material"} w? "}"]
3 "entity" ["(" w? s?.(["," w?]){{
    ["name" w? ":" w? t!"name"]
    ["material" w? ":" w? t!"material"]
}} w? ")"]
4 "entities" ["entities" w? ":" w? "[" w? s?.(["," w?]){@"entity""entity"} w? "]"]
5 "document" [w? ?"Scene" w? "(" w? s?(["," w?]){{
    @"materials""materials"
    @"entities""entities"
}} w? ")" w?]
    "#;
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = bootstrap::convert(
        &parse(&bootstrap::rules(), rules).unwrap(),
        &mut vec![] // stores ignored meta data
    ).unwrap();

    let data = parse(&rules, text);
    match data {
        Err((range, err)) => {
            // Report the error to standard error output.
            ParseErrorHandler::new(&text).error(range, err);
        }
        _ => {}
    }

    bencher.iter(|| {
        let _ = parse(&rules, text).unwrap();
    });
}
