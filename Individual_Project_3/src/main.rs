use csv::ReaderBuilder;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{prelude::*, OwnedRepr};
use ndarray_csv::Array2Reader;


fn main() {
    // load data
    let train = load_data("data/train.csv");
    let test = load_data("data/test.csv");

    // print info
    println!(
        "training with {} samples, testing with {} samples, {} features and {} target",
        train.nsamples(),
        test.nsamples(),
        train.nfeatures(),
        train.ntargets()
    );

    // train model
    println!("training and testing model...");
    // let model = LogisticRegression::default()
    let mut max_accuracy_confusion_matrix = iterate_with_values(&train, &test, 0.01, 100);
    let mut best_threshold = 0.0;
    let mut best_max_iterations = 0;
    let mut threshold = 0.02;

    // iterate over threshold and max_iterations to find best accuracy
    for max_iterations in (1000..5000).step_by(500) {
        while threshold < 1.0 {
            let confusion_matrix = iterate_with_values(&train, &test, threshold, max_iterations);

            if confusion_matrix.accuracy() > max_accuracy_confusion_matrix.accuracy() {
                max_accuracy_confusion_matrix = confusion_matrix;
                best_threshold = threshold;
                best_max_iterations = max_iterations;
            }
            threshold += 0.01;
        }
        threshold = 0.02;
    }

    // print results
    println!(
        "most accurate confusion matrix: {:?}",
        max_accuracy_confusion_matrix
    );
    println!(
        "with max_iterations: {}, threshold: {}",
        best_max_iterations, best_threshold
    );
    println!("accuracy {}", max_accuracy_confusion_matrix.accuracy(),);
    println!("precision {}", max_accuracy_confusion_matrix.precision(),);
    println!("recall {}", max_accuracy_confusion_matrix.recall(),);
}

fn iterate_with_values(
    train: &DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>>,
    >,
    test: &DatasetBase<
        ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
        ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 2]>>,
    >,
    threshold: f64,
    max_iterations: u64,
) -> ConfusionMatrix<&'static str> {
    let model = LogisticRegression::default()
        .max_iterations(max_iterations)
        .gradient_tolerance(0.0001)
        .fit(train)
        .expect("can train model");

    let validation = model.set_threshold(threshold).predict(test);

    let confusion_matrix = validation
        .confusion_matrix(test)
        .expect("can create confusion matrix");

    confusion_matrix
}

fn load_data(path: &str) -> Dataset<f64, &'static str> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path(path)
        .expect("can create reader");

    let array: Array2<f64> = reader
        .deserialize_array2_dynamic()
        .expect("can deserialize array");

    let (data, targets) = (
        array.slice(s![.., 0..2]).to_owned(),
        array.column(2).to_owned(),
    );

    let feature_names = vec!["test 1", "test 2"];

    Dataset::new(data, targets)
        .map_targets(|x| {
            if *x as usize == 1 {
                "accepted"
            } else {
                "denied"
            }
        })
        .with_feature_names(feature_names)
}
