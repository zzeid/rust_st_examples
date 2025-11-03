/* fn main() {
    println!("Hello, world!");
} */

// EN: Minimal example: connect → SELECT (abc, area, perimeter) → HTML report
// LV: Minimāls piemērs: pieslēgums → SELECT (abc, area, perimeter) → HTML atskaite

use postgres::{Client, NoTls};                    // EN: Postgres client / LV: Postgres klients
use std::error::Error;                            // EN: Error trait for main() Result / LV: Kļūdu tips main() rezultātam
use std::fs::File;                                // EN: For creating a file / LV: Faila izveidei
use std::io::Write;                               // EN: For writing bytes/text / LV: Rakstīšanai failā

const DBPARAMETRI: &str =
    "host=localhost user=postgres password=postgres dbname=postgres";
// EN: Built-in connection string (adjust if needed)
// LV: Iebūvēta savienojuma virkne (pielāgo, ja vajag)

const OUTPUT_HTML: &str = "poligoni_report_rust.html";
// EN: Output HTML file name
// LV: HTML faila nosaukums

fn main() -> Result<(), Box<dyn Error>> {
    // EN: 1) Connect to PostgreSQL (requires PostGIS + table test.poligoni_polygons)
    // LV: 1) Pieslēgties PostgreSQL (vajag PostGIS + tabulu test.poligoni_polygons)
    let mut client = Client::connect(DBPARAMETRI, NoTls)?;

    // EN: 2) Run one SELECT that computes area/perimeter on the fly
    // LV: 2) SELECT, kas uzreiz aprēķina platību/perimetru
    let rows = client.query(
        r#"
        SELECT
          abc,                                         -- EN/LV: polygon code (A/B/C) / poligona kods
          ST_Area(geom)/1000000::float8   AS area,     -- EN: polygon area       LV: poligona platība [KM^2]
          ST_Perimeter(geom)/1000::float8 AS perimeter -- EN: polygon perimeter  LV: poligona perimetrs [KM]
        FROM test.poligoni_polygons
        ORDER BY abc;
        "#,
        &[],
    )?;

    // EN: 3) Create HTML file to write the report
    // LV: 3) Izveidot HTML failu, kurā rakstīt atskaiti
    let mut f = File::create(OUTPUT_HTML)?;

    // EN: 4) Write minimal HTML header + table head
    // LV: 4) Uzrakstīt minimālo HTML galveni + tabulas virsrakstus
    writeln!(
        f,
        r#"<!doctype html>
<html lang="en"><head>
<meta charset="utf-8">
<title>Polygon parametrs</title>
<style>
body{{font-family:Segoe UI,Arial,sans-serif;margin:20px}}
table{{border-collapse:collapse}}
th,td{{border:1px solid #ccc;padding:6px 10px}}
th{{background:#f6f6f6}}
</style>
</head><body>
<h1>Daudzstūru parametri</h1>
<h1>Polygon parametrs</h1>
<table>
<thead><tr><th>abc</th><th>platība (km^2)</th><th>perimetrs (km)</th></tr></thead>
<thead><tr><th>abc</th><th>area (km^2)</th><th>perimeter (km)</th></tr></thead>
<tbody>
"#
    )?;

    // EN: 5) For each DB row → extract values → append a <tr> to HTML
    // LV: 5) Katrai DB rindai → paņemt vērtības → pievienot <tr> HTML failam
    for row in rows {
        let abc: String = row.get("abc");      // EN: text label / LV: teksta apzīmējums
        let area: f64 = row.get("area");       // EN: numeric area / LV: skaitliska platība
        let per: f64 = row.get("perimeter");   // EN: numeric perimeter / LV: skaitlisks perimetrs

        // EN: Basic HTML escaping for abc (safety); numbers are safe as-is
        // LV: Vienkārša HTML aizsardzība abc laukam; skaitļi droši kā ir
        writeln!(
            f,
            "<tr><td>{}</td><td>{:.1}</td><td>{:.1}</td></tr>",
            abc, area, per
        )?;
    }

    // EN: 6) Close HTML tags and finish the file
    // LV: 6) Aizvērt HTML tagus un pabeigt failu
    writeln!(f, "</tbody></table></body></html>")?;

    // EN: 7) Tell user where the file is
    // LV: 7) Parādīt ceļu līdz failam
    println!("HTML saved to {}", OUTPUT_HTML);

    // EN: 8) Done
    // LV: 8) Gatavs
    Ok(())
}

