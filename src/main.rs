use std::{path::Path, sync::{Arc, Mutex}, time::{Duration, Instant}};

use device_query::{DeviceEvents, DeviceEventsHandler};
use rusqlite::Connection;

fn main(){
    println!("From here the loggin begins, it doesn't run in the background but just compiles and runs for 10 seconds");

    let event_handler = DeviceEventsHandler::new(Duration::from_millis(10)).expect("Failed to start an event loop");
    let db_path = "keylog.db";
    let db_exists = Path::new(db_path).exists();
    let conn  = match Connection::open(db_path){
        Ok(val) => {
            println!("Database connection created");
            val
        }
        Err(e) => return println!("Error connecting to the database: {}",e)
    };
    let mouse_count = Arc::new(Mutex::new(0));
    let mouse_count_clone =  mouse_count.clone();
    let keys_count = Arc::new(Mutex::new(0));
    let keys_count_clone = keys_count.clone();
    let last_mouse_posn = Arc::new(Mutex::new((0,0)));
    let total_distance_travelled = Arc::new(Mutex::new(0.0));
    let total_distance_travelled_clone = total_distance_travelled.clone();

    // creating database if it not exists
    if !db_exists{
        let result  = conn.execute("
            CREATE TABLE keylog(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                key_presses INTEGER NOT NULL,
                mouse_clicks INTEGER NOT NULL,
                mouse_distance REAL NOT NULL
            )",());

        match result {
            Ok(val) => println!("Sqlite database created successfully"),
            Err(e) => println!("The error occured while creating a database is : {}", e)
        }
    }

    // Register event to key press
    let _key_press_guard = event_handler.on_key_down(move|key|{

        // taking key count
        let mut key_count = match keys_count_clone.lock(){
            Ok(val) => val,
            Err(poisoned) => {
                println!("Mutex was poisoned, recovering...");
                poisoned.into_inner()
            }
        };
        *key_count += 1;

    });


    // Register event to mouse click
    let _key_mouse_click = event_handler.on_mouse_down(move |click|{
        // taking the mouse_count reference
        let mut mouse_count_val = match mouse_count_clone.lock(){
            Ok(val) => val,
            Err(poisoned) => {
                println!("Mutex was poisoned, recovering...");
                poisoned.into_inner()
            }
        };
        *mouse_count_val += 1;
    });

    // Register to mouse move
    let _key_mouse_move = event_handler.on_mouse_move(move |click|{
        let mut last_posn = match last_mouse_posn.lock(){
            Ok(val) => val,
            Err(poisoned) => {
                println!("Mutex was poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        let mut total_distance = match total_distance_travelled.lock(){
            Ok(val) => val,
            Err(poisoned) => {
                println!("Mutex was poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        let x_diff = click.0 - last_posn.0;
        let y_diff = click.1 - last_posn.1;

        let distance_travelled = ((x_diff * x_diff + y_diff * y_diff) as f64).sqrt();

        last_posn.0 = click.0;
        last_posn.1 = click.1;
        *total_distance += distance_travelled;

    });



    let mut start_time = Instant::now();
    let duration = Duration::from_secs(10);
    loop {

        if start_time.elapsed() >= duration{

            // taking the keys_count reference
            let mut keys_count_val = match keys_count.lock(){
                Ok(val) => val,
                Err(poisoned) => {
                    println!("Mutex was poisoned, recovering...");
                    poisoned.into_inner()
                }
            };

            // taking the mouse_count reference
            let mut mouse_count_val = match mouse_count.lock(){
                Ok(val) => val,
                Err(poisoned) => {
                    println!("Mutex was poisoned, recovering...");
                    poisoned.into_inner()
                }
            };


            // taking the distance_travelled reference
            let mut total_distance_travelled_val = match total_distance_travelled_clone.lock() {
                Ok(val) => val,
                Err(poisoned) => {
                    println!("Mutex was poisoned, recovering...");
                    poisoned.into_inner()
                }
            };

            println!("Keys pressed: {}", *keys_count_val);
            println!("Mouse clicks: {}", *mouse_count_val);
            println!("Total distance travelled: {}", *total_distance_travelled_val);
            println!("Time elapsed: {:?}", start_time.elapsed());
            
            
            println!("Inserting data into the database..."); 
            let _ = conn.execute(
                "INSERT INTO keylog (timestamp, key_presses, mouse_clicks, mouse_distance) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![
                    start_time.elapsed().as_secs(),
                    *keys_count_val,
                    *mouse_count_val,
                    *total_distance_travelled_val
                ]
            );

            *mouse_count_val = 0;
            *keys_count_val = 0;
            *total_distance_travelled_val = 0.0;
            start_time = Instant::now();
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

