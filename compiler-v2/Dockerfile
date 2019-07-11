FROM python:3

WORKDIR /app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY src /app/src
COPY data /data

ENV DATA_DIR=/data

CMD [ "pipenv", "run", "python", "/app/src/main.py" ]
