pub mod prediction {
	use std::error::Error;

	use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};
	const FACTOR: f32 = 1.7647058823529411;

	// --------------- DEVICE ---------------

	#[derive(Clone, Debug)]
	pub struct ModelPath {
		pub sec_dim: String,
		pub flow: String,
		pub freq: String,
	}

	#[derive(Clone, Copy, Debug, Eq, PartialEq)]
	pub enum Device {
		CH100,
		CH190,
		CH270,
	}

	#[derive(Debug)]
	pub struct Model {
		bundle: SavedModelBundle,
		method: String,
		input_param: String,
		output_param: String,
		graph: Graph,
	}

	impl Device {
		pub const ALL: [Self; 3] = [
			Device::CH100,
			Device::CH190,
			Device::CH270, // Device::CH190(ModelPath {
			               // 	sec_dim: "".to_owned(),
			               // 	flow: "".to_owned(),
			               // 	freq: "".to_owned(),
			               // }),
			               // Device::CH270(ModelPath {
			               // 	sec_dim: "".to_owned(),
			               // 	flow: "".to_owned(),
			               // 	freq: "".to_owned(),
			               // }),
		];

		pub fn model_path(&self) -> ModelPath {
			let (a, b, c) = match self {
				Device::CH100 => (
					"model_secondnumberdevice1",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQDEVICEONE",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQDEVICEONE",
				),
				Device::CH190 => (
					"model_secondnumberdevice2",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQ",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQPREDICT",
				),
				Device::CH270 => (
					"model_secondnumberdevice3",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQDEVICETHREE",
					"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQDEVICETHREE",
				),
			};

			ModelPath {
				sec_dim: a.to_string(),
				flow: b.to_string(),
				freq: c.to_string(),
			}
		}

		pub fn max_value(&self) -> f32 {
			use Device::{CH100, CH190, CH270};
			match self {
				CH100 => 100.0,
				CH190 => 190.0,
				CH270 => 270.0,
			}
		}
	}

	// impl Default for Device<'_> {
	// 	fn default() -> Device {
	// 		Device::CH190
	// 	}
	// }

	impl std::fmt::Display for Device {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(
				f,
				"{}",
				match self {
					Device::CH100 => "100",
					Device::CH190 => "190",
					Device::CH270 => "270",
				}
			)
		}
	}

	// --------------- PREDICTION ---------------

	pub fn load_model(model_path: &str) -> Model {
		// create a new graph
		let mut graph = Graph::new();
		println!("{model_path}");

		let (input_param, out_param) = match model_path {
			"model_secondnumberdevice1" => ("dense_8_input", "dense_11"),
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQDEVICEONE" => {
				("dense_25_input", "dense_28")
			}
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQDEVICEONE" => {
				("dense_33_input", "dense_36")
			}
			"model_secondnumberdevice2" => ("dense_input", "dense_3"),
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQ" => {
				("dense_154_input", "dense_157")
			}
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQPREDICT" => {
				("dense_162_input", "dense_165")
			}
			"model_secondnumberdevice3" => ("dense_8_input", "dense_11"),
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQDEVICETHREE" => {
				("dense_1_input", "dense_4")
			}
			"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQDEVICETHREE" => {
				("dense_5_input", "dense_8")
			}
			_ => panic!("Could not infer input and output parameters for the model."),
		};

		let models_location = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/models/trained_models/");

		let model_path = format!("{models_location}{model_path}");
		// load the saved model as a graph
		let bundle =
			SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_path)
				.expect("Can't load saved model");

		Model {
			bundle,
			graph,
			method: "serving_default".to_string(),
			output_param: out_param.to_string(),
			input_param: input_param.to_string(),
		}
	}

	fn predict(
		model: &Model,
		method: &str,
		// input_param: &str,
		// output_param: &str,
		args: Vec<f32>,
	) -> Result<Tensor<f32>, Box<dyn Error>> {
		let input_tensor: Tensor<f32> = Tensor::new(&[1, *&args.len() as u64])
			.with_values(&args)
			.unwrap();

		let Model {
			bundle: model_bundle,
			graph,
			..
		} = model;

		let Model {
			input_param,
			output_param,
			..
		} = model;

		//Initiate a session
		let session = &model_bundle.session;

		//The values will be fed to and retrieved from the model with this
		let mut args = SessionRunArgs::new();

		//Retrieve the pred functions signature
		let signature_train = model_bundle.meta_graph_def().get_signature(method).unwrap();

		let input_info_pred = signature_train.get_input(input_param).unwrap();

		let output_info_pred = signature_train.get_output(output_param).unwrap();

		let input_op_pred = graph
			.operation_by_name_required(&input_info_pred.name().name)
			.unwrap();

		let output_op_pred = graph
			.operation_by_name_required(&output_info_pred.name().name)
			.unwrap();

		args.add_feed(&input_op_pred, 0, &input_tensor);

		let out = args.request_fetch(&output_op_pred, 0);

		//Run the session
		session.run(&mut args)?;

		let prediction = args.fetch(out)?;

		Ok(prediction)
	}

	pub fn compute(
		input: f32,
		sec_dim_model: &Model,
		flow_model: &Model,
		freq_model: &Model,
	) -> Result<(f32, f32, f32, f32), Box<dyn Error>> {
		let major_axis = input / FACTOR;
		// Step1: using the major axis, predict the minor axis.
		let minor_axis = predict(
			sec_dim_model,
			"serving_default",
			// "dense_input",
			// "dense_3",
			vec![major_axis],
		)
		.expect("compute failed")[0];

		// Step2: using the major and minor axis, predict the flow rate
		let flow_rate: Tensor<f32> = predict(
			flow_model,
			"serving_default",
			// "dense_154_input",
			// "dense_157",
			vec![major_axis, minor_axis],
		)
		.expect("compute failed");

		let pbs = flow_rate[0];
		let fluo_surf = flow_rate[1];

		// Step3: using the two flow rates, predict the frequency
		let frequency: f32 = predict(
			freq_model,
			"serving_default",
			// "dense_162_input",
			// "dense_165",
			vec![pbs, fluo_surf],
		)
		.expect("compute failed")[0];

		// let elapsed_time = now.elapsed().as_micros();

		println!(
			"
        Dimension A                 : {input:>8.2}
        Dimension B                 : {minor_axis:>8.2}
        Flow rate PBS (μl/min)      : {pbs:>8.2}
        Flow rate FluoSurf (μl/min) : {fluo_surf:>8.2}
        Estimated frequency         : {frequency:>8.2}
        "
		);

		Ok((minor_axis, pbs, fluo_surf, frequency))
	}

	// fn main() {
	// 	// Model paths
	// 	let sec_dim_path = "models/trained_models/model_secondnumberdevice2";
	// 	let flow_rate_path =
	// 		"models/trained_models/newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQ";
	// 	let freq_path =
	// 		"models/trained_models/newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQPREDICT";
	//
	// 	// let input: f32 = 50.0;
	//
	// 	// let major_axis = input / FACTOR;
	//
	// 	// load the models
	//
	// 	let sec_dim_model = load_model(sec_dim_path);
	// 	let flow_model = load_model(flow_rate_path);
	// 	let freq_model = load_model(freq_path);
	// }
}
