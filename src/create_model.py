import tensorflow as tf
from tensorflow import keras
from keras import callbacks
from keras import layers
import onnxmltools

# for reading and parsing csv
import numpy as np
from numpy import genfromtxt


def new_model(csv_path, model_name):
    print("CSV path: " + csv_path)
    print("Saving model at: " + model_name)

    data = genfromtxt(
        csv_path,
        delimiter=",",
        skip_header=1,
        dtype=np.float64,
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

    # Addition callback for training progress
    epoch_print_callback = callbacks.LambdaCallback(
        on_epoch_end=lambda epoch, logs: print(f"\033[1A\033[K{epoch}")
    )

    # print("Fit model on training data")
    history = model.fit(
        x_train,
        y_train,
        batch_size=10,
        validation_split=0.1,
        epochs=100,
        callbacks=[epoch_print_callback],
        verbose=0,
    )

    onnx_model = onnxmltools.convert_keras(model)
    onnxmltools.utils.save_model(onnx_model, model_name + ".onnx")
