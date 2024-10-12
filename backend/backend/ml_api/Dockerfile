# ml_api/Dockerfile
FROM python:3.10-slim

# Atualiza e instala as dependências do sistema
RUN apt-get update && apt-get install -y \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Define o diretório de trabalho
WORKDIR /app

# Copia o arquivo de requisitos e instala as dependências do Python
COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

# Copia o restante do código para o contêiner
COPY . .

# Comando padrão para rodar a aplicação
CMD ["python", "app.py"]  # Substitua por seu arquivo principal
