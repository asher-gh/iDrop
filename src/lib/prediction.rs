use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};

enum Device {
	CH100,
	CH190,
	CH270,
}

// mod predic {
// 	// Model paths
// 	let minor_axis_model = "models/trained_models/model_secondnumberdevice2";
// 	let flow_rate_model =
// 		"models/trained_models/newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQ";
// 	let frequency_model =
// 		"models/trained_models/newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQPREDICT";
//
// 	const FACTOR: f32 = 1.7647058823529411;
//
// 	let mut major_axis: f32 = 50.0;
//
// 	major_axis /= FACTOR;
//
// 	// Step1: using the major axis, predict the minor axis.
// 	let minor_axis: f32 = predict(
// 		minor_axis_model,
// 		"serving_default",
// 		"dense_input",
// 		"dense_3",
// 		vec![major_axis],
// 	)[0];
//
// 	// Step2: using the major and minor axis, predict the flow rate
// 	let flow_rate: Tensor<f32> = predict(
// 		flow_rate_model,
// 		"serving_default",
// 		"dense_154_input",
// 		"dense_157",
// 		vec![major_axis, minor_axis],
// 	);
// 	let pbs = flow_rate[0];
// 	let fluo_surf = flow_rate[1];
//
// 	// Step3: using the two flow rates, predict the frequency
// 	let frequency: f32 = predict(
// 		frequency_model,
// 		"serving_default",
// 		"dense_162_input",
// 		"dense_165",
// 		vec![pbs, fluo_surf],
// 	)[0];
//
// 	println!(
// 		"
//         Minor axis                  : {minor_axis:>8.2}
//         Flow rate PBS (μl/min)      : {pbs:>8.2}
//         Flow rate FluoSurf (μl/min) : {fluo_surf:>8.2}
//         Estimated frequency         : {frequency:>8.2}
//         "
// 	);
//
//
// }

enum Device {
	A = "100",
}

pub fn predict(
	model_path: &str,
	method: &str,
	input_param: &str,
	output_param: &str,
	args: Vec<f32>,
) -> Tensor<f32> {
	let input_tensor: Tensor<f32> = Tensor::new(&[1, *&args.len() as u64])
		.with_values(&args)
		.unwrap();
	//TODO: ("figure out what the &[1,2] is");

	// create a new graph
	let mut graph = Graph::new();

	// load the saved model as a graph
	let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, model_path)
		.expect("Can't load saved model");

	//Initiate a session
	let session = &bundle.session;

	//The values will be fed to and retrieved from the model with this
	let mut args = SessionRunArgs::new();

	//Retrieve the pred functions signature
	let signature_train = bundle.meta_graph_def().get_signature(method).unwrap();

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
	session
		.run(&mut args)
		.expect("Error occurred during calculations");

	let prediction = args.fetch(out).unwrap();

	prediction[0]
}
