FROM python:3

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY src /app/src
COPY data /data

ENV DATA_DIR=/data

CMD [ "python", "/app/src/main.py" ]
