import joblib
import os

def import_model():
    loaded_linear_reg = joblib.load('./model/ridge_reg_model_sklearn.joblib')
    loaded_ridge_reg = joblib.load('./model/ridge_reg_model_sklearn.joblib')
    loaded_scaler = joblib.load('./model/standard_scaler_sklearn.joblib')
    return "LOADED MODEL"


if __name__ == "__main__":
    print(import_model())
