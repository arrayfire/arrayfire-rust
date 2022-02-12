use self::{
    ann::ArtificialNeuralNetworkBuilder, arrayfire_mnist::ArrayfireNormalizedMnist, model::Model,
};

use arrayfire::Array;

use mnist::MnistBuilder;

pub(crate) mod arrayfire_mnist {
    use arrayfire::{dim4, transpose, Array};
    use mnist::NormalizedMnist;

    pub const TRAINING_SET_SIZE: u32 = 6_000;
    pub const VALIDATION_SET_SIZE: u32 = 1_000;
    pub const TEST_SET_SIZE: u32 = 1_000;
    pub const IMAGE_DIMENSION: usize = 28;
    pub struct ArrayfireNormalizedMnist {
        pub training_labels: Array<u8>,
        pub training_images: Array<f32>,
        pub validation_labels: Array<u8>,
        pub validation_images: Array<f32>,
        pub testing_labels: Array<u8>,
        pub testing_images: Array<f32>,
    }

    impl From<&NormalizedMnist> for ArrayfireNormalizedMnist {
        fn from(mnist_data: &NormalizedMnist) -> Self {
            let training_images = Array::new(
                &mnist_data.trn_img,
                dim4![
                    (IMAGE_DIMENSION * IMAGE_DIMENSION) as u64,
                    TRAINING_SET_SIZE as u64,
                    1,
                    1
                ],
            );
            let training_labels = Array::new(
                &mnist_data.trn_lbl,
                dim4![10, TRAINING_SET_SIZE as u64, 1, 1],
            );

            let testing_images = Array::new(
                &mnist_data.tst_img,
                dim4![
                    (IMAGE_DIMENSION * IMAGE_DIMENSION) as u64,
                    TEST_SET_SIZE as u64,
                    1,
                    1
                ],
            );
            let testing_labels =
                Array::new(&mnist_data.tst_lbl, dim4![10, TEST_SET_SIZE as u64, 1, 1]);

            let validation_images = Array::new(
                &mnist_data.val_img,
                dim4![
                    (IMAGE_DIMENSION * IMAGE_DIMENSION) as u64,
                    VALIDATION_SET_SIZE as u64,
                    1,
                    1
                ],
            );
            let validation_labels = Array::new(
                &mnist_data.val_lbl,
                dim4![10, VALIDATION_SET_SIZE as u64, 1, 1],
            );

            ArrayfireNormalizedMnist {
                training_labels: transpose(&training_labels, false),
                training_images: transpose(&training_images, false),
                validation_labels: transpose(&validation_labels, false),
                validation_images: transpose(&validation_images, false),
                testing_labels: transpose(&testing_labels, false),
                testing_images: transpose(&testing_images, false),
            }
        }
    }
}
pub(crate) mod model {
    use arrayfire::Array;
    pub trait Model {
        fn predict(&self, feature: &Array<f32>) -> Array<f32>;

        fn train(
            &mut self,
            training_features: &Array<f32>,
            training_labels: &Array<u8>,
            validation_features: &Array<f32>,
            validation_labels: &Array<u8>,
            learning_rate_alpha: f64,
            max_epochs: i32,
            batch_size: i32,
            max_err: f64,
        ) -> f64;
    }
}
mod ann {
    use super::model::Model;
    use arrayfire::{
        constant, dim4, index, join, matmul, seq, sigmoid, sum_all, transpose, Array, MatProp,
    };
    pub struct ArtificialNeuralNetwork {
        weights: Vec<Array<f32>>,
        num_layers: usize,
    }

    pub struct ArtificialNeuralNetworkBuilder {
        feature_width: usize,
        layer_widths: Vec<usize>,
        label_width: usize,
        initial_random_range: f32,
        num_layers: usize,
    }

    impl ArtificialNeuralNetworkBuilder {
        pub fn new() -> ArtificialNeuralNetworkBuilder {
            ArtificialNeuralNetworkBuilder {
                feature_width: 0,
                label_width: 0,
                num_layers: 2,
                layer_widths: vec![],
                initial_random_range: 0.05,
            }
        }

        pub fn with_feature_width(
            &mut self,
            feature_width: usize,
        ) -> &mut ArtificialNeuralNetworkBuilder {
            self.feature_width = feature_width;
            self
        }

        pub fn with_label_width(
            &mut self,
            label_width: usize,
        ) -> &mut ArtificialNeuralNetworkBuilder {
            self.label_width = label_width;
            self
        }

        pub fn add_hidden_layer(
            &mut self,
            layer_width: usize,
        ) -> &mut ArtificialNeuralNetworkBuilder {
            self.layer_widths.push(layer_width);
            self.num_layers += 1;
            self
        }

        pub fn with_initial_random_range(
            &mut self,
            range: f32,
        ) -> &mut ArtificialNeuralNetworkBuilder {
            self.initial_random_range = range;
            self
        }

        pub fn build(&self) -> ArtificialNeuralNetwork {
            let mut widths: Vec<usize> = vec![];
            widths.push(self.feature_width);
            widths.append(&mut self.layer_widths.clone());
            widths.push(self.label_width);

            let mut weights = vec![];
            for index in 0..(widths.len() - 1) {
                weights.push(
                    self.initial_random_range
                        * arrayfire::randu::<f32>(dim4![
                            widths[index] as u64 + 1,
                            widths[index + 1] as u64,
                            1,
                            1
                        ])
                        - self.initial_random_range / 2f32,
                )
            }
            ArtificialNeuralNetwork {
                weights,
                num_layers: self.num_layers,
            }
        }
    }

    impl Model for ArtificialNeuralNetwork {
        fn train(
            &mut self,
            training_features: &Array<f32>,
            training_labels: &Array<u8>,
            validation_features: &Array<f32>,
            validation_labels: &Array<u8>,
            learning_rate_alpha: f64,
            max_epochs: i32,
            batch_size: i32,
            max_err: f64,
        ) -> f64 {
            let number_of_training_samples = training_features.dims()[0] as i32;
            let number_of_batches_in_training_set = number_of_training_samples / batch_size;

            let validation_batch_size = 1;

            let _number_of_validation_samples = validation_features.dims()[0] as i32;
            let number_of_batches_in_validation_set = 1; //number_of_validation_samples / validation_batch_size;

            let mut avg_error = 0f64;

            for epoch in 0..max_epochs {
                let mut errors = Vec::with_capacity(number_of_batches_in_validation_set as usize);
                for training_batch in 0..number_of_batches_in_training_set {
                    let start = training_batch * batch_size;
                    let end = start + batch_size - 1;

                    let features = index(training_features, &[seq![start, end, 1], seq!()]);
                    let labels = index(training_labels, &[seq![start, end, 1], seq!()]);

                    let signals = self.forward_propagate(&features);

                    self.back_propagate(&signals, &labels, learning_rate_alpha);
                }

                for validation_batch in 0..number_of_batches_in_validation_set {
                    let start = validation_batch * validation_batch_size;
                    let end = start + validation_batch_size - 1;

                    let prediction =
                        self.predict(&index(validation_features, &[seq![start, end, 1], seq!()]));

                    let target = &index(validation_labels, &[seq![start, end, 1], seq!()]);

                    errors.push(Self::error(&prediction, target));
                }

                avg_error = errors.clone().into_iter().sum::<f64>()
                    / (number_of_batches_in_validation_set) as f64;
                if avg_error < max_err {
                    println!("Converged on Epoch: {}", epoch + 1);
                    return avg_error;
                }

                if (epoch + 1) % 10 == 0 {
                    println!("Epoch: {}, Error: {}", epoch + 1, avg_error);
                }
            }
            avg_error
        }

        fn predict(&self, input: &Array<f32>) -> Array<f32> {
            let signal = self.forward_propagate(input);
            signal.last().unwrap().copy()
        }
    }

    impl ArtificialNeuralNetwork {
        fn forward_propagate(&self, input: &Array<f32>) -> Vec<Array<f32>> {
            //first layer is the input
            let mut signal = Vec::with_capacity(self.num_layers);

            signal.push(input.copy());

            for layer_index in 0..(self.num_layers - 1) {
                let signal_with_bias: Array<f32> = Self::add_bias(&signal[layer_index]);
                let output = matmul(
                    &signal_with_bias,
                    &self.weights[layer_index],
                    MatProp::NONE,
                    MatProp::NONE,
                );
                signal.push(sigmoid(&output));
            }

            signal
        }

        fn add_bias(array: &Array<f32>) -> Array<f32> {
            join(
                1,
                &constant::<f32>(1f32, dim4![array.dims()[0], 1, 1, 1]),
                array,
            )
        }

        fn deriv(out: &Array<f32>) -> Array<f32> {
            out * (1 - out)
        }

        fn back_propagate(
            &mut self,
            signals: &Vec<Array<f32>>,
            labels: &Array<u8>,
            learning_rate_alpha: f64,
        ) {
            let mut output = signals.last().unwrap();
            let mut error = output - labels;

            let m = labels.dims()[0] as i32;

            for layer_index in (0..self.num_layers - 1).rev() {
                let signal = Self::add_bias(&signals[layer_index]);
                let delta = transpose(&(Self::deriv(output) * error), false);

                let tg =
                    learning_rate_alpha * matmul(&delta, &signal, MatProp::NONE, MatProp::NONE);
                let gradient = -(tg) / m;
                self.weights[layer_index] += transpose(&gradient, false);

                output = &signals[layer_index];

                let err = &matmul(
                    &transpose(&delta, false),
                    &transpose(&self.weights[layer_index], false),
                    MatProp::NONE,
                    MatProp::NONE,
                );

                error = index(err, &[seq!(), seq!(1, output.dims()[1] as i32, 1)]);
            }
        }

        fn error(prediction: &Array<f32>, target: &Array<u8>) -> f64 {
            let dif = (prediction - target) * 1f64;
            let sum = sum_all(&(&dif * &dif)).0;
            sum.sqrt()
        }
    }
}

fn accuracy(predicted: &Array<f32>, target: &Array<f32>) -> f32 {
    let (_target_maximums, target_max_indices) = arrayfire::imax(target, 1);
    let (_predicted_maximums, predicted_max_indices) = arrayfire::imax(predicted, 1);

    let (matches, _) = arrayfire::count_all(&arrayfire::eq(
        &target_max_indices,
        &predicted_max_indices,
        false,
    ));
    100f32 * matches as f32 / target_max_indices.elements() as f32
}

fn main() {
    arrayfire::info();
    println!("** ArrayFire-Rust ANN Demo **\n");

    let mnist = MnistBuilder::new()
        .label_format_one_hot()
        .training_set_length(50_000)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .download_and_extract()
        .finalize()
        .normalize();

    let mnist = ArrayfireNormalizedMnist::from(&mnist);

    let mut model = ArtificialNeuralNetworkBuilder::new()
        .with_feature_width(28 * 28)
        .with_label_width(10)
        .add_hidden_layer(100)
        .add_hidden_layer(50)
        .with_initial_random_range(0.05f32)
        .build();

    model.train(
        &mnist.training_images,
        &mnist.training_labels,
        &mnist.validation_images,
        &mnist.validation_labels,
        2.0,
        250,
        100,
        0.05,
    );

    let train_output = model.predict(&mnist.training_images);
    let test_output = model.predict(&mnist.testing_images);

    arrayfire::sync(arrayfire::get_device());

    println!("\nTraining set:");
    println!(
        "Accuracy on training data: {}",
        accuracy(&train_output, &(mnist.training_labels * 1f32)),
    );

    println!("\nTest set:");
    println!(
        "Accuracy on testing  data: {}",
        accuracy(&test_output, &(mnist.testing_labels * 1f32)),
    );
}
