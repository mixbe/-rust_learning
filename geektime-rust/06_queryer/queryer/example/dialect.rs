
use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {

    tracing_subscriber::fmt::init();

    let sql = "SELECT a a1, b, 123, myfunc(b), * \
    FROM data_source \
    WHERE a >b AND b<100 and c BETWEEN 10 AND 20 \
    ORDER BY  a DESC, b \
    LIMIT 50 OFFSET 10";

    // SQL 解析
    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:#?}", ast);
}