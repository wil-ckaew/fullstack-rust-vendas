import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
import joblib
from flask import Flask, request, jsonify

# Inicializando o Flask
app = Flask(__name__)

# Simula alguns dados de vendas (este código deve ser ajustado para funcionar em produção)
data = {
    'product_id': [1, 2, 3, 1, 2, 3, 1, 2],
    'quantity_sold': [30, 40, 50, 60, 70, 80, 90, 100],
    'days_since_last_restock': [10, 15, 20, 10, 15, 20, 10, 15]
}

df = pd.DataFrame(data)

# Dividindo dados em treino e teste
X = df[['quantity_sold', 'days_since_last_restock']]
y = df['days_since_last_restock']

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2)

# Treinando o modelo
model = LinearRegression()
model.fit(X_train, y_train)

# Salvando o modelo
joblib.dump(model, 'restock_model.pkl')

@app.route('/predict_restock', methods=['POST'])
def predict_restock():
    input_data = request.get_json()
    product_id = input_data.get('product_id')
    quantity_sold = input_data.get('quantity_sold')
    days_since_last_restock = input_data.get('days_since_last_restock')

    # Prepare os dados para previsão
    data = pd.DataFrame({
        'quantity_sold': [quantity_sold],
        'days_since_last_restock': [days_since_last_restock]
    })

    # Carregar o modelo e fazer a previsão
    model = joblib.load('restock_model.pkl')
    prediction = model.predict(data)

    return jsonify({'days_until_restock': prediction[0]})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000, debug=True)
