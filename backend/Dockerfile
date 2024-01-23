FROM python:3.10-alpine

WORKDIR /app
COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt
COPY . .
EXPOSE 5000
ENV FLASK_RUN_HOST=0.0.0.0
CMD ["flask", "run", "--debug"]
