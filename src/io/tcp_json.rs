use tokio::net::tcp::TcpStream;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec, LinesCodec};
use serde_json::{Value, Error};
use tokio_serde_json::{ReadJson, WriteJson};


// Type alias for returning JSON stream
type DeserializedStream = ReadJson<FramedRead<TcpStream, LinesCodec>, serde_json::Value>;
type SerializedStream = WriteJson<FramedWrite<TcpStream, LengthDelimitedCodec>, serde_json::Value>;

#[derive(Deserialize, Debug)]
pub enum CommandType {
	Up,
	Down,
	Left,
	Right,
}

// A struct for providing stong types to deserialize the incoming JSONs
#[derive(Deserialize, Debug)]
pub struct JsonMsg {
	order_type: String,
	device: String,
	cmd: u64,   
}

impl JsonMsg {
	pub fn serializer(socket: TcpStream) -> SerializedStream {
		// Delimit frames using a length header
	    let length_delimited = FramedWrite::new(socket, LengthDelimitedCodec::new());

	    // Serialize frames
	    let serializer = WriteJson::new(length_delimited);

	    serializer
	}

	pub fn deserialize(socket: TcpStream) ->  DeserializedStream {
		// Delimit frames using a length header
	    let length_delimited = FramedRead::new(socket, LinesCodec::new());

	    // Deserialize frames
	    let deserialized = ReadJson::<_, Value>::new(length_delimited);

	    deserialized
	}

	// Deserialize JSON and process contents of json packet
	pub fn process_msg(msg: serde_json::Value) -> Result<JsonMsg, Error> {
		let typed_json: JsonMsg = serde_json::from_value(msg)?;//.expect("Couldn't make JSON");
		// Parse JSON body into enums compatible with flow market
		// match typed_json.order_type.to_lowercase().as_ref() {
		// 	"move" => println!("{:?}", typed_json.cmd),
		// 	"stop" => println!("{:?}", typed_json.cmd),
		// 	_ => {
		// 			println!("Entered an invalid ordertype!");
		// 			return None;
		// 		},
		// };
		Ok(typed_json)
	}
}










