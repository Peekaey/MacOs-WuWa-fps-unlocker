
use rusqlite::{Connection, Result};

// Query to create a database trigger that will update the CustomFrameRate value to the equivalent of 120 FPS (3)
// Enum options of 0 (30 FPS), 1 (45 FPS), 2 (60 FPS), and 3 (120 FPS)
// Due to the reliance of needing a PlayMenuInfo and MenuData key pair, we will stick to setting the value to 120
fn get_insert_customframerate_trigger_query() -> String {
    let query = r#"
        CREATE TRIGGER Ensure120CustomFrameRate
        AFTER UPDATE OF value on LocalStorage
        WHEN NEW.key = 'CustomFrameRate' AND NEW.value != 120
        BEGIN
            UPDATE LocalStorage SET value = 120 WHERE key = 'CustomFrameRate';
        END;
    "#.to_string();
    
    return query;
}

fn get_delete_customframerate_trigger_query() -> String {
    let query = r#"
        DROP TRIGGER IF EXISTS Ensure120CustomFrameRate;
    "#.to_string();
    
    return query;
}

pub fn get_update_customframerate_query() -> String {
    let query = r#"
        UPDATE LocalStorage SET value = 120 WHERE key = 'CustomFrameRate';
    "#.to_string();
    
    return query;
}

fn get_delete_dependency_keypair_query() -> String {
    let query = r#"
        DELETE FROM LocalStorage WHERE key IN ('PlayMenuInfo', 'MenuData');
    "#.to_string();
    
    return query;
}

fn get_insert_menudata_quer() -> String {
    let query = r#"
    INSERT INTO LocalStorage (key, value)
    VALUES ('MenuData', '{"___MetaType___":"___Map___","Content":[[1,42],[2,72],[3,0],[4,45],[5,0],[6,0],[7,-1],[10,3],[11,3],[20,0],[21,0],[22,0],[23,0],[24,0],[25,0],[26,0],[27,0],[28,0],[29,0],[30,0],[31,0],[32,0],[33,0],[34,0],[35,0],[36,0],[37,0],[38,0],[39,0],[40,0],[41,0],[42,0],[43,0],[44,0],[45,0],[46,0],[47,0],[48,0],[49,0],[50,0],[51,1],[52,2],[53,0],[54,3],[55,2],[56,2],[57,1],[58,1],[59,1],[61,0],[62,0],[63,1],[64,1],[65,0],[66,0],[69,54],[70,56],[79,2],[87,1],[88,0],[89,69],[90,69],[91,76],[92,74],[93,0],[99,100],[100,100],[101,0],[102,0],[103,1],[104,50],[105,0],[106,0.3],[107,0],[112,0],[113,0],[114,0],[115,0],[116,0],[117,0],[118,0],[119,0],[120,0],[121,1],[122,1],[123,0],[130,0],[131,0],[132,1],[135,1],[133,0]]}');
    "#.to_string();
    
    return query;
}

fn get_insert_playmenuinfo_query() -> String { 
    let query = r#"
    INSERT INTO LocalStorage (key, value)
    VALUES ('PlayMenuInfo', '"{\"1\":45,\"2\":72,\"3\":0,\"4\":45,\"5\":0,\"6\":0,\"7\":-1,\"10\":3,\"11\":3,\"20\":0,\"21\":0,\"22\":0,\"23\":0,\"24\":0,\"25\":0,\"26\":0,\"27\":0,\"28\":0,\"29\":0,\"30\":0,\"31\":0,\"32\":0,\"33\":0,\"34\":0,\"35\":0,\"36\":0,\"37\":0,\"38\":0,\"39\":0,\"40\":0,\"41\":0,\"42\":0,\"43\":0,\"44\":0,\"45\":0,\"46\":0,\"47\":0,\"48\":0,\"49\":0,\"50\":0,\"51\":1,\"52\":2,\"53\":0,\"54\":3,\"55\":2,\"56\":2,\"57\":1,\"58\":1,\"59\":1,\"61\":0,\"62\":0,\"63\":1,\"64\":1,\"65\":0,\"66\":0,\"69\":54,\"70\":56,\"79\":2,\"87\":1,\"88\":0,\"89\":50,\"90\":51,\"91\":86,\"92\":87,\"93\":0,\"99\":100,\"100\":100,\"101\":0,\"102\":0,\"103\":1,\"104\":50,\"105\":0,\"106\":0.3,\"107\":0,\"112\":0,\"113\":0,\"114\":0,\"115\":0,\"116\":0,\"117\":0,\"118\":0,\"119\":0,\"120\":0,\"121\":1,\"122\":1,\"123\":0,\"130\":0,\"131\":0,\"132\":1}"');
    "#.to_string();
    
    return query;
}

pub fn execute_update_max_refreshrate(file_path: &String) -> Result<(), String> {
    // Open a connection to the LocalStorage sqlite database
    let connection = Connection::open(file_path);

    return match connection {
        Ok(conn) => {

            // Delete existing update trigger before creating new one
            let delete_trigger_query = get_delete_customframerate_trigger_query();
            if let Err(e) = conn.execute(&delete_trigger_query, []) {
                eprintln!("Error deleting existing trigger: {}", e);
                return Err(format!("Error deleting existing trigger: {}", e));
            }

            // Execute the trigger creation query
            let create_trigger_query = get_insert_customframerate_trigger_query();
            if let Err(e) = conn.execute(&create_trigger_query, []) {
                eprintln!("Error creating trigger: {}", e);
                return Err(format!("Error creating trigger: {}", e));
            }

            // Execute the update query to set CustomFrameRate to 120 FPS
            let update_query = get_update_customframerate_query();
            if let Err(e) = conn.execute(&update_query, []) {
                eprintln!("Error updating CustomFrameRate: {}", e);
            }

            // Execute the delete dependency keypair query
            let delete_dependency_query = get_delete_dependency_keypair_query();
            if let Err(e) = conn.execute(&delete_dependency_query, []) {
                eprintln!("Error deleting PlayMenuInfo & MenuData keypair: {}", e);
            }
            // Insert MenuData 
            let insert_menudata_query = get_insert_menudata_quer();
            if let Err(e) = conn.execute(&insert_menudata_query, []) {
                eprintln!("Error inserting MenuData: {}", e);
                return Err(format!("Error inserting MenuData: {}", e));
            }
            // Insert PlayMenuInfo
            let insert_playmenuinfo_query = get_insert_playmenuinfo_query();
            if let Err(e) = conn.execute(&insert_playmenuinfo_query, []) {
                eprintln!("Error inserting PlayMenuInfo: {}", e);
                return Err(format!("Error inserting PlayMenuInfo: {}", e));
            }

            Ok(())
        },
        Err(error) => {
            eprintln!("Failed to open database connection: {}", error);
            Err(format!("Failed to open database connection: {}", error))
        }
    }
}



