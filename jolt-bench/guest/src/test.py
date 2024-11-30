import joblib
import numpy as np
import os

# Load models and scaler
linear_reg = joblib.load('./model/linear_regression_model.joblib')
ridge_reg = joblib.load('./model/ridge_regression_model.joblib')
ridge_poly = joblib.load('./model/polynomial_ridge_regression_model.joblib')
scaler = joblib.load('./model/scaler_model.joblib')

def predict_all_batch(features_list):
    """
    Predict using all three models for a batch of feature sets.
    
    :param features_list: List of feature lists
    :return: JSON string with predictions
    """
    X = np.array(features_list)
    X_scaled = scaler.transform(X)
    
    linear_pred = linear_reg.predict(X_scaled).tolist()
    ridge_pred = ridge_reg.predict(X_scaled).tolist()
    ridge_poly_pred = ridge_poly.predict(X_scaled).tolist()
    
    results = {
        'linear_regression': linear_pred,
        'ridge_regression': ridge_pred,
        'polynomial_ridge_regression': ridge_poly_pred
    }
    
    return json.dumps(results)

if __name__ == "__main__":

    print("testing...")
