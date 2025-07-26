# Automation-API-Reqres
Automation-API-Reqres is an API automation repo using Katalon Studio and Reqres. Covers testing for GET list users, GET single user, PUT update, and POST register (email from single user).



# Katalon API Automation - Reqres.in

This project demonstrates API automation testing using **Katalon Studio Free 9.5.0** for the [Reqres.in](https://reqres.in/) demo API.

## 📁 Project Structure

```
Test Cases/
└── ApiReqres/
    ├── TCGetListUser
    ├── TCGetViewSingleUser
    ├── TCRegisterSuccesfull
    └── TCUpdateUser

Test Suites/
└── TSApiReqres

Include/
└── jsonSchema/
    ├── requestRegisterSuccesfull.json
    ├── responseGetListUser.json
    ├── responseGetViewSingleUser.json
    ├── responseRegisterSuccessfull.json
    └── responseUpdateUser.json
```

## 🔧 Tools Used

- **Katalon Studio Free 9.5.0**
- GitHub (for version control)
- Reqres.in public API

## 📌 Endpoints Covered

1. **GET List Users**  
   Endpoint: `/api/users?page=1`  
   Test Case: `TCGetListUser`

2. **GET Single User**  
   Endpoint: `/api/users/{id}`  
   Test Case: `TCGetViewSingleUser`

3. **POST Register Successful**  
   Endpoint: `/api/register`  
   Test Case: `TCRegisterSuccesfull`  
   > Email is dynamically fetched from `GET Single User` response.

4. **PUT Update User**  
   Endpoint: `/api/users/{id}`  
   Test Case: `TCUpdateUser`

## ✅ Test Suite

All test cases are grouped and executed in the `TSApiReqres` test suite.

## 📄 JSON Schema Validation

All API responses are validated using JSON schema files located in `Include/jsonSchema/` for strict response structure validation.

## 🚀 How to Run

1. Open the project using Katalon Studio 9.5.0.
2. Open test suite `TSApiReqres`.
3. Click **Run** to execute all test cases.
4. Review console and report output for results.

