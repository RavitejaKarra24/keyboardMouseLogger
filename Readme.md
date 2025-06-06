# Keyboard & Mouse Logger

A Rust-based application that logs keyboard and mouse activity, storing the data in a SQLite database for analysis and monitoring purposes.

## Features

- **Keyboard Tracking**: Counts key presses in real-time
- **Mouse Click Monitoring**: Tracks mouse button clicks
- **Mouse Movement Analysis**: Calculates total distance traveled by the mouse cursor
- **SQLite Database Storage**: Persistent storage of activity data with timestamps
- **Configurable Logging Intervals**: Currently set to 10-second intervals
- **Automatic Database Creation**: Creates the database schema if it doesn't exist

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Dependencies

The project uses the following crates:

- `device_query` - For capturing keyboard and mouse events
- `rusqlite` - For SQLite database operations

## Installation

1. Clone or download the project
2. Navigate to the project directory:
   ```bash
   cd keyboard_mouse_logger
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:

```bash
cargo run
```

The application will:

1. Start monitoring keyboard and mouse activity
2. Create a `keylog.db` SQLite database file if it doesn't exist
3. Log activity data every 10 seconds
4. Display real-time statistics in the console

### Output Example

```
From here the logging begins, it doesn't run in the background but just compiles and runs for 10 seconds
Database connection created
Keys pressed: 25
Mouse clicks: 8
Total distance travelled: 1234.56
Time elapsed: 10s
Inserting data into the database...
```

## Database Schema

The application creates a `keylog` table with the following structure:

```sql
CREATE TABLE keylog(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    key_presses INTEGER NOT NULL,
    mouse_clicks INTEGER NOT NULL,
    mouse_distance REAL NOT NULL
);
```

## Configuration

- **Logging Interval**: Currently hardcoded to 10 seconds (line with `Duration::from_secs(10)`)
- **Event Polling**: Set to 10ms intervals for event detection
- **Database File**: Saved as `keylog.db` in the current directory

## Privacy & Security Notice

⚠️ **Important**: This application monitors and logs keyboard and mouse activity. Ensure you:

- Have appropriate permissions to monitor the system
- Use this tool responsibly and in compliance with local laws
- Inform users if running on shared systems
- Secure the database file as it may contain sensitive timing information

## Platform Support

This application should work on systems supported by the `device_query` crate, typically including:

- Windows
- macOS
- Linux

## Limitations

- Currently runs for continuous logging (infinite loop)
- No built-in data export functionality
- No GUI interface (command-line only)
- Fixed 10-second logging intervals

## Future Enhancements

- Configurable logging intervals
- Data export to CSV/JSON
- Background service mode
- GUI interface for data visualization
- Filtering and analysis tools

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]
