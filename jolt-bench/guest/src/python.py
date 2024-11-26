import joblib
import os

def import_model():
    print(os.getcwd());
    loaded_linear_reg = joblib.load('./model/ridge_reg_model_sklearn.joblib')
    loaded_ridge_reg = joblib.load('./model/ridge_reg_model_sklearn.joblib')
    loaded_scaler = joblib.load('./model/standard_scaler_sklearn.joblib')
    print("Loaded")
