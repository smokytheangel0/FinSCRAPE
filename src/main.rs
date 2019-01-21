//this will be used to parse json into structs
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
use serde_json::{Value, from_str};
use serde::{Deserialize, Serialize};

//this will be used to get json from server
extern crate reqwest;

//this will be used to create and add to databases
/* HARDEST TASK */
extern crate rusqlite;
use rusqlite::{Connection, NO_PARAMS};

//this will be used to query the storage devices available
extern crate systemstat;

//this will send notifications to operator email
extern crate lettre;

//this will be used to read/write csv files
extern crate csv;

use std::fs;
use std::env;
use std::option;
use std::result;
use std::time::Instant;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

//before running this as production, the pi should be set up running off the usb A port on the powerbar
//with the hard drive plugged in to the AC port. should use the 80gb spinner and the 64gb usb as the initial
//pair of hot swaps, this will give you a month on each

 #[allow(bad_style)]

/*
fn set_disk(mut master: &DB, mut metrics: &DB) {
    //check conf file for current master dir in binary directory
    //get a list of storage devices
    //get a list of storage utilization and capacity
    //if current disk is > 2/3 full, notify() and
    //excluding the current master disk, find a disk that has greater than 33 gb capacity
    //if none continue
    //if some set master path field to new disk, notify(changed master path)

    //leave a note of current dir in binary directory
    //set path in master struct

    //conf file should be blank on first run,
    //and at the beginnig, if blank, use first disk larger than 33 gb

    //set_labels()
}
*/

/*
fn notify(reason: &Notify) {
    //this will send emails or other correspondence to the operator
}
*/


fn get_data() -> (HashMap<String, CryptoFiat>, u64) {
    let mut json = "".to_string();
    let mut timestamp = 0;
    loop {
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if timestamp % 30 == 0 {
            json = reqwest::get("https://min-api.cryptocompare.com/data/pricemultifull?fsyms=BTC,ETH,BCH,LTC,EOS,BNB,XMR,DASH,VEN,NEO,ETC,ZEC,WAVES,BTG,DCR,REP,GNO,MCO,FCT,HSR,DGD,XZC,VERI,PART,GAS,ZEN,GBYTE,BTCD,MLN,XCP,XRP,MAID&tsyms=USD&api_key={6cbc5ffe92ca7113e33a5f379e8d73389d6f8a1ba30d10a003135826b0f64815}")
                .expect("the request to the cryptocompare api failed")
                .text().expect("unable to get text from the cryptocompare api response");

            break
        }
    }

    let mut frame = HashMap::new();
    let data: Value = serde_json::from_str(&json).expect("unable to convert response text to untyped object");
    let object = data.as_object().expect("unable to convert outer values to map");
    let object = object["RAW"].as_object().expect("unable to convert inner values to map");
    for crypto in object.keys() {
        for fiat in object[crypto].as_object().unwrap().keys() {
            let pair_block: CryptoFiat = serde_json::from_value(object[crypto][fiat].clone()).expect("failed to convert untyped map to typed struct");
            frame.entry(format!("{}and{}", crypto, fiat)).or_insert(pair_block);
        }
    }

    (frame, timestamp)

}

fn write_data(frame: &HashMap<String, CryptoFiat>, timestamp: &u64, master: DB) {
    //for each write, do checks if master, table, etc exist
    //that way if the disk is changed it can write a new master
    //rather than loosing a row
    //for pair in frame.keys():
    //  writeVEC = arrange_vec(frame[pair], timestamp)
    //  create table called pair if none
    //  write new row to table using writevec
    //set_labels() if the write to single master takes to long, this can be par_itered with multiple dbs instead of tables
    //if new master/tables etc then notify(new master established) as soon as the first row is written
    //that should be the safe to unmount notification for the previous drive

    let storage = Connection::open(master.path.unwrap()).expect("failed to open or create master");
    for table_name in frame.keys() {
            let table_statement = format!("CREATE TABLE {} (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )", table_name);
            storage.execute(&table_statement, NO_PARAMS).expect("failed to create table");
    }

    storage.close();
}

fn arrange_vec(pair: &CryptoFiat, timestamp: &u64) -> Vec<String> {
    let mut writeVEC: Vec<String> = vec![];
    writeVEC.push(timestamp.to_string());
    writeVEC.push(pair.last_update.to_string());
    writeVEC.push(pair.price.to_string());
    writeVEC.push(pair.last_market.to_string());
    writeVEC.push(pair.last_volume_crypto.to_string());
    writeVEC.push(pair.volume_hour_crypto.to_string()); 
    writeVEC.push(pair.volume_day_crypto.to_string());
    writeVEC.push(pair.volume_24_hour_crypto.to_string());
    writeVEC.push(pair.total_volume_24_hour_crypto.to_string());
    writeVEC.push(pair.last_volume_fiat.to_string());
    writeVEC.push(pair.volume_hour_fiat.to_string());
    writeVEC.push(pair.volume_day_fiat.to_string());
    writeVEC.push(pair.volume_24_hour_fiat.to_string());
    writeVEC.push(pair.total_volume_24_hour_fiat.to_string());
    writeVEC.push(pair.change_day.to_string());
    writeVEC.push(pair.change_pct_day.to_string());
    writeVEC.push(pair.change_24_hour.to_string());
    writeVEC.push(pair.change_pct_24_hour.to_string());
    writeVEC.push(pair.supply.to_string());
    writeVEC.push(pair.market_cap.to_string());
    writeVEC.push(pair.open_hour.to_string());
    writeVEC.push(pair.high_hour.to_string());
    writeVEC.push(pair.low_hour.to_string());
    writeVEC.push(pair.open_day.to_string());
    writeVEC.push(pair.high_day.to_string());
    writeVEC.push(pair.low_day.to_string());
    writeVEC.push(pair.open_24_hour.to_string());
    writeVEC.push(pair.high_24_hour.to_string());
    writeVEC.push(pair.low_24_hour.to_string());
    writeVEC
}

/* 5th
fn queue_frames(mut queue: HashMap<String, Vec<Vec<String>>>, 
                frame: &HashMap<String, CryptoFiat>, 
                timestamp: &u64
                ) -> HashMap<String, Vec<Vec<String>>> {
    //this should read the agent conf file and set window_size and interval
    //push each new frame to the queue until the queue is == 10 frames
    //then remove the 0th frame each time a frame is pushed to the queue

    //it should get a writeVEC for each pair in the frame
    //then assemble the writeVECS in the following fashion

    //for pair in frame.keys():
    //  let writeVEC = arrange_vec(frame[pair], timestamp)
    //  if queue[pair][-1][0] - writeVEC[0] >= interval:
    //      queue[pair].push(writeVEC)
    //      if queue[pair].len() > window_size:
    //          queue[pair].remove(0)
    //
    //queue is hashmap<String, Vec<Vec<String>>> (
    //                                      "BTCandUSD": [writeVEC0, writeVEC1], 
    //                                      "ETH-USD": [writeVEC0, writeVEC1]
    //                                    )
    //with each subkey a hashmap (of different pairs) at a different timestamp
}
*/

/* 3rd
fn set_labels(mut metricVEC: Vec<u64>, duration: u64) -> Vec<u64> {
    //this will be called from inside functions to update the metrics struct
    //framestamp, set_disk, get_agent_config, get_data, queue_frames, inform_agent, write_data, agent_action, main_loop
    //each field will be an int calculated by timecomplete - timestart
    
}
*/

/* 4th
fn measure(metricVEC: Vec<u64>, master: DB) {
    //for each write do checks if master, table, etc exist
    //that way if the disk is changed it can write a new master
    //rather than loosing a row

    //framestamp, storage_device, set_disk, get_agent_config, get_data, queue_frames, inform_agent, write_data, agent_action, main_loop
    //each field will be an int calculated by timecomplete - timestart

    //another rust script could be created which goes over the metrics database
    //and notifies if things get out of bounds or exceed expectations (usually not for free)
}
*/

/* 6th
fn inform_agent(queue: &HashMap<String, Vec<Vec<String>>>) {
    //this should write a csv file named by each key in queue
    //write hardcoded header
    //one writevec per line following that, comma seperated per index
    //using the required coin pair's csv, the agent should read these and act whenever a new timestamp is found at the last index
    //the agent should check every 2-5s
    //and set a third file with a read->action_complete pair of time stamps for metrics
    //set_labels()
}
*/

/* 7th
fn get_agent_metrics() {
    //this will read the agent's read->action_complete timestamps and replace the previous ones
    //the agent's metric file should have headings framestamp, duration
    //the agent should push to this file instead of rewriting, so that this function
    //can update a previous row if the agents metric is missing
    //set_labels()
}
*/

enum Notify {
    ChangedDB,
    FirstWrite,
    LowStorage,
    ChangedConfig
}

struct DB {
    path: Option<String>,
    storage_device: Option<String>

}

fn default_string() -> String {
    "MISSING".to_string()
}
fn default_int() -> i64 {
    424242
}
fn default_float() -> f64 {
    4242.42
}

#[derive(Serialize, Deserialize)]
struct CryptoFiat {
    //data["RAW"]["$CRYPTO"]["$FIAT"]
    //this is where we put the json after it is broken down untyped into crypto-fiat pairs
    #[serde(rename="TYPE")]
    #[serde(default="default_string")]
    class: String,
    #[serde(rename="MARKET")]
    #[serde(default="default_string")]
    market:String,
    #[serde(rename="FROMSYMBOL")]
    #[serde(default="default_string")]
    crypto_symbol: String,
    #[serde(rename="TOSYMBOL")]
    #[serde(default="default_string")]
    fiat_symbol:String,
    #[serde(rename="FLAGS")]
    #[serde(default="default_string")]
    flags: String,
    #[serde(rename="PRICE")]
    #[serde(default="default_float")]
    price: f64,
    #[serde(rename="LASTUPDATE")]
    #[serde(default="default_int")]
    last_update: i64,
    #[serde(rename="LASTVOLUME")]
    #[serde(default="default_float")]
    last_volume_crypto: f64,
    #[serde(rename="LASTVOLUMETO")]
    #[serde(default="default_float")]
    last_volume_fiat: f64,
    #[serde(skip_deserializing)]
    //this one comes out as a string sometimes
    LASTTRADEID: i64,
    #[serde(rename="VOLUMEDAY")]
    #[serde(default="default_float")]
    volume_day_crypto: f64,
    #[serde(rename="VOLUMEDAYTO")]
    #[serde(default="default_float")]
    volume_day_fiat: f64,
    #[serde(rename="VOLUME24HOUR")]
    #[serde(default="default_float")]
    volume_24_hour_crypto: f64,
    #[serde(rename="VOLUME24HOURTO")]
    #[serde(default="default_float")]
    volume_24_hour_fiat: f64,
    #[serde(rename="OPENDAY")]
    #[serde(default="default_float")]
    open_day: f64,
    #[serde(rename="HIGHDAY")]
    #[serde(default="default_float")]
    high_day: f64,
    #[serde(rename="LOWDAY")]
    #[serde(default="default_float")]
    low_day: f64,
    #[serde(rename="OPEN24HOUR")]
    #[serde(default="default_float")]
    open_24_hour: f64,
    #[serde(rename="HIGH24HOUR")]
    #[serde(default="default_float")]
    high_24_hour: f64,
    #[serde(rename="LOW24HOUR")]
    #[serde(default="default_float")]
    low_24_hour: f64,
    #[serde(rename="LASTMARKET")]
    #[serde(default="default_string")]
    last_market: String,
    //this and the nearly all the following
    //have no data in a or all currencies other than USD
    #[serde(rename="VOLUMEHOUR")]
    #[serde(default="default_float")]
    volume_hour_crypto: f64,
    #[serde(rename="VOLUMEHOURTO")]
    #[serde(default="default_float")]
    volume_hour_fiat: f64,
    #[serde(rename="OPENHOUR")]
    #[serde(default="default_float")]
    open_hour: f64,
    #[serde(rename="HIGHHOUR")]
    #[serde(default="default_float")]
    high_hour: f64,
    #[serde(rename="LOWHOUR")]
    #[serde(default="default_float")]
    low_hour: f64,
    #[serde(rename="CHANGE24HOUR")]
    #[serde(default="default_float")]
    change_24_hour: f64,
    #[serde(rename="CHANGEPCT24HOUR")]
    #[serde(default="default_float")]
    change_pct_24_hour: f64,
    #[serde(rename="CHANGEDAY")]
    #[serde(default="default_float")]
    change_day: f64,
    #[serde(rename="CHANGEPCTDAY")]
    #[serde(default="default_float")]
    change_pct_day: f64,
    #[serde(rename="SUPPLY")]
    #[serde(default="default_float")]
    supply: f64,
    #[serde(rename="MKTCAP")]
    #[serde(default="default_float")]
    market_cap: f64,
    #[serde(rename="TOTALVOLUME24H")]
    #[serde(default="default_float")]
    total_volume_24_hour_crypto: f64,
    #[serde(rename="TOTALVOLUME24HTO")]
    #[serde(default="default_float")]
    total_volume_24_hour_fiat: f64,
    #[serde(default="default_string")]
    IMAGEURL: String

}

fn main() {
//perf: keys can be str, 
//vecs and hashmaps all have length known, and can be defined

    let mut master = DB{
        path: None,
        storage_device: None
    };

    let mut metrics = DB{
        path: None,
        storage_device: None
    };

    let mut queue: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    loop{
        let mut metricVEC: Vec<i64> = vec![];
        let start = Instant::now();
        //set_disk(&master, &metrics);
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //let (frame, timestamp) = get_data();
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //queue = queue_frames(queue, &frame, &timestamp);
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //inform_agent(&queue);
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //write_data(&frame, &timestamp);
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        let start = Instant::now();
        //get_agent_metrics();
        let duration = start.elapsed().as_secs();
        //metricVEC = set_labels(metricVEC, duration);

        //measure(metricVEC, metrics);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //utils
    fn get_fake_data()-> (HashMap<String, CryptoFiat>, u64) {
        let json = fs::read_to_string("response_crypto.txt")
        .expect("Something went wrong reading the file");

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let mut frame = HashMap::new();
        let data: Value = serde_json::from_str(&json).expect("unable to convert response text to untyped object");
        let object = data.as_object().expect("unable to convert outer values to map");
        let object = object["RAW"].as_object().expect("unable to convert inner values to map");
        for crypto in object.keys() {
            for fiat in object[crypto].as_object().unwrap().keys() {
                let pair_block: CryptoFiat = serde_json::from_value(object[crypto][fiat].clone()).expect("failed to convert untyped map to typed struct");
                frame.entry(format!("{}and{}", crypto, fiat)).or_insert(pair_block);
            }
        }

        (frame, timestamp)


    }

    #[test]
    fn get_fake_data_returns_valid_frame() {
        let (frame, timestamp) = get_fake_data();
        if frame["BTCandUSD"].crypto_symbol != "BTC" ||
           frame["BTCandUSD"].fiat_symbol != "USD" {
               panic!("get_fake_data returned an invalid frame");
           }
    }



    //unit tests
    #[test]
    fn set_disk_group(){
        panic!("not implemented");
    }


    #[test]
    fn notify_group(){
        panic!("not implemented");
    }


    fn get_data_sleeps_till_30() -> Result<(), ()>{
        let (frame, timestamp) = get_data();
        if timestamp % 30 == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn get_data_creates_valid_frame() -> Result<(), ()> {
        let (frame, timestamp) = get_data();
        if frame["BTCandUSD"].crypto_symbol == "BTC" &&
           frame["BTCandUSD"].fiat_symbol == "USD"
        {
            Ok(())
        }
        else {
            Err(())
        }
    }

    fn get_data_frame_has_all_crypto() -> Result<(), ()> {
        let (frame, timestamp) = get_data();
        if frame.len() == 32 {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    #[ignore]
    fn get_data_group(){
        get_data_sleeps_till_30().expect("the request did not happen on a round 30 seconds");
        get_data_creates_valid_frame().expect("get_data returned an invalid frame");
        get_data_frame_has_all_crypto().expect("frame does not contain enough crypto-USD pairs");
    }


    fn arrange_vec_has_29_items() -> Result<(), ()> {
        let (frame, timestamp) = get_fake_data();
        let pair = &frame["BTCandUSD"];
        let writeVEC = arrange_vec(&pair, &timestamp);
        if writeVEC.len() == 29 {
            Ok(())
        } else {
            Err(())
        }
    }

    fn arrange_vec_returns_valid_writevec() -> Result<(), ()> {
        let (frame, timestamp) = get_fake_data();
        let pair = &frame["BTCandUSD"];
        let writeVEC = arrange_vec(&pair, &timestamp);
        if writeVEC[0].len() == 10 &&
            //market
           writeVEC[3] == "Coinbase" &&
           //volume24h
           writeVEC[7] == "37533.51939446323" &&
           //volume_day_fiat
           writeVEC[11] == "140675918.74609685" &&
           //change_pct_day
           writeVEC[15] == "2.3316949881989917" &&
           //market_cap
           writeVEC[19] == "65291977762.5" &&
           //low_24h
           writeVEC[28] == "3643.41"
        {
            Ok(())
        } else {
            Err(())
        }
    }

    #[test]
    fn arrange_vec_test_group(){
        arrange_vec_has_29_items().expect("arrange_vec returns an incorrect number of items");
        arrange_vec_returns_valid_writevec().expect("arrange_vec returns an invalid writeVEC")
    }


    fn write_data_creates_db_when_none() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        //get paths
        let cargo = env::current_dir().expect("unable to find current dir");
        let db_path = cargo.to_str().expect("path is invalid unicode");

        let filesInSrc = fs::read_dir(&db_path).expect("failed to read contents of download directory");

        for fileNAME in filesInSrc {
            let entry = fileNAME.expect("DirEntry returned 0");
            let fileNAME: String = entry.file_name()
                                //this converts the OSstr into a string slice
                                .into_string()
                                .expect("the file_name could not be converted to a string")
                                //this converts the string slice into an owned string
                                .to_owned().clone();

            if fileNAME.contains("test.db") {
                fs::remove_file(&entry.path()).expect("failed to remove file after match");
            }
        }

        let (frame, timestamp) = get_fake_data();
        write_data(&frame, &timestamp, master);

        let filesInSrc = fs::read_dir(&db_path).expect("failed to read contents of download directory");

        for fileNAME in filesInSrc {
            let entry = fileNAME.expect("DirEntry returned 0");
            let fileNAME: String = entry.file_name()
                                //this converts the OSstr into a string slice
                                .into_string()
                                .expect("the file_name could not be converted to a string")
                                //this converts the string slice into an owned string
                                .to_owned().clone();

            if fileNAME.contains("test.db"){                
                fs::remove_file(&entry.path()).expect("failed to remove file after match");
                return Ok(());
            }
        }


        return Err(());
    }

    fn write_data_adds_valid_tables_to_db() -> Result <(), ()> {
        let master = DB {
            path: Some("test.db".to_string()),
            storage_device: None
        };

        let (frame, timestamp) = get_fake_data();
        write_data(&frame, &timestamp, master);
        //BTC,ETH,BCH,LTC,EOS,BNB,XMR,DASH,VEN,NEO,ETC,ZEC,WAVES,BTG,DCR,REP,GNO,MCO,FCT,HSR,DGD,XZC,VERI,PART,GAS,ZEN,GBYTE,BTCD,MLN,XCP,XRP,MAID
        let storage = Connection::open("test.db").expect("failed to open the database");
        let mut table_vec: HashSet<String> = [].iter().cloned().collect();
        let expect_vec: HashSet<String> = [
                                            "BCHandUSD".to_string(),
                                            "BNBandUSD".to_string(),
                                            "BTCDandUSD".to_string(),
                                            "BTCandUSD".to_string(),
                                            "BTGandUSD".to_string(),
                                            "DASHandUSD".to_string(),
                                            "DCRandUSD".to_string(),
                                            "DGDandUSD".to_string(),
                                            "EOSandUSD".to_string(),
                                            "ETCandUSD".to_string(),
                                            "ETHandUSD".to_string(),
                                            "FCTandUSD".to_string(),
                                            "GASandUSD".to_string(),
                                            "GBYTEandUSD".to_string(),
                                            "GNOandUSD".to_string(),
                                            "HSRandUSD".to_string(),
                                            "LTCandUSD".to_string(),
                                            "MAIDandUSD".to_string(),
                                            "MCOandUSD".to_string(),
                                            "MLNandUSD".to_string(),
                                            "NEOandUSD".to_string(),
                                            "PARTandUSD".to_string(),
                                            "REPandUSD".to_string(),
                                            "VENandUSD".to_string(),
                                            "VERIandUSD".to_string(),
                                            "WAVESandUSD".to_string(),
                                            "XCPandUSD".to_string(),
                                            "XMRandUSD".to_string(),
                                            "XRPandUSD".to_string(),
                                            "XZCandUSD".to_string(),
                                            "ZECandUSD".to_string(),
                                            "ZENandUSD".to_string()
                                            ].iter().cloned().collect();
        
        let mut statement = storage.prepare("SELECT name FROM sqlite_master WHERE type='table';").expect("failed to prepare statement");
        //this returns 0 items, which is why it fails later on
        let table_iter = statement.query_map(NO_PARAMS, |row| {
            let pair: String = row.get(0);
            table_vec.insert(pair.to_owned());
        });

        //table_vec.len() is 0
        if expect_vec == table_vec {
            Ok(())
        } else {
            Err(())
        }
        
    }

    fn write_data_adds_valid_row_to_table() -> Result <(), ()> {
        Err(())
    }

    #[test]
    fn write_data_group(){
        write_data_creates_db_when_none().expect("write_data failed to create master");
        write_data_adds_valid_tables_to_db().expect("write_data failed to add tables to DB");
    }


    #[test]
    fn queue_frames_group(){
        panic!("not implemented");
    }


    #[test]
    fn set_labels_group(){
        panic!("not implemented");
    }


    #[test]
    fn measure_group(){
        panic!("not implemented");
    }


    #[test]
    fn inform_agent_group(){
        panic!("not implemented");
    }


    #[test]
    fn get_agent_metrics_group(){
        panic!("not implemented");
    }


    #[test]
    fn get_agent_config_group(){
        panic!("not implemented");
    }
}