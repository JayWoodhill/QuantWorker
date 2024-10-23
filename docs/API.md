# Decision-Making Engine API

## Overview
This microservice is responsible for processing real-time trade data from the pseudo-ASX API and generating trading signals based on predefined models. It uses FastAPI as the web framework and supports inter-service communication via ZeroMQ.

---

## Endpoints

### 1. Health Check

- **`GET /health`**
- **Description**: This endpoint returns the status of the Decision-Making Engine service, ensuring it's running and ready to process incoming trades.
- **Response**:
  ```json
  {
      "status": "Decision Engine is running"
  }
