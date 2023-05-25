from keras import layers
from tensorflow import keras
import onnxmltools

# for reading and parsing csv
from numpy import genfromtxt

csv_path = ""


def new_model(csv_path, model_name="new_model"):
    data = genfromtxt(
        csv_path,
        delimiter=",",
        skip_header=1,
    )

    x_train = data[:, [0, 1, 4]]
    y_train = data[:, [2, 3]]

    model = keras.Sequential(name="flow_prediction_nn")
    model.add(layers.Dense(300, input_shape=(3,), activation="sigmoid"))
    model.add(layers.Dense(100, kernel_initializer="he_uniform", activation="relu"))
    model.add(layers.Dense(1000, kernel_initializer="he_uniform", activation="relu"))
    model.add(layers.Dense(1000, kernel_initializer="he_uniform", activation="relu"))
    model.add(layers.Dense(100, kernel_initializer="he_uniform", activation="relu"))
    model.add(layers.Dense(2))

    model.compile(
        optimizer="adam",
        loss="mae",
    )

    # print("Fit model on training data")
    history = model.fit(
        x_train,
        y_train,
        batch_size=10,
        validation_split=0.1,
        epochs=10,
    )

    # convert the generated TF model to onnx format
    onnx_model = onnxmltools.convert_keras(model)
    # save the newly created model
    # default location: ../assets/models/new_model.onnx
    onnxmltools.utils.save_model(onnx_model, model_name + ".onnx")
