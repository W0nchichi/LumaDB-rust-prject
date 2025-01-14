//luma_db/lumadb_parser/src/command.rs

//an enum to define all of the commands that can be taken by the parser

#[derive(Debug)]
//just implementing SELECT for now
pub enum command {
    //will give examples above all of the cases
    //Select *
    //Select Example.id, Example.sid
    Select {
        columns: Vec<String>, //Example.id, Example.sid, *
    },
    //Where E = 1
    //Where E = 1 AND D = 2
    //Where E = 1 OR (D = 2 AND F = 3)
    //because of the complexity of the Whereclause, let's just make another enum for it
    Where {
        conditions: Option<WhereClause>, //<Con 1> AND <Con 2>
    },
    //From <Table>
    //From <Table> <T>
    From {
        table: String,
        alias: Option<char>,
    },

}

#[derive(Debug)]
pub enum WhereClause {
    SingleCondition {
        column: String,         // Column name
        operator: String,       // Comparison operator (e.g., =, >, <)
        value: String,          // Comparison value
    },
    CompoundCondition {
        conditions: Vec<WhereClause>, // Nested conditions
        conjunction: String,         // Logical operator (e.g., AND, OR)
    },
}