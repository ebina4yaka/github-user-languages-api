# 🦀GithubUserLanguagesAPI🦀
## About
This is a web API, that uses the Github GraphQL API to calculate and return programming language usage for 100 repositories for a given Github user. (exclude forked repositories)
## Usage
### Request
```
https://github-user-languages-api.herokuapp.com/user/{username}
```
example
```
https://github-user-languages-api.herokuapp.com/user/ebina4yaka
```
### Response
```
[
  {
    "name": string,
    "percentage": number
  }
]
```
example
```Json
[
  {
    "name": "TypeScript",
    "percentage": 80.13
  },
  {
    "name": "Rust",
    "percentage": 11.1
  },
  {
    "name": "JavaScript",
    "percentage": 4.48
  },
  {
    "name": "Elm",
    "percentage": 2.2
  },
  {
    "name": "CSS",
    "percentage": 1.34
  },
  {
    "name": "Dockerfile",
    "percentage": 0.75
  }
]
```
### Hide individual languages

You can use `?hide=language1,language2` parameter to hide individual languages.

example
```
https://github-user-languages-api.herokuapp.com/user/ebina4yaka?hide=css,html,dockerfile
```

## Deploy
### Add .env file
```Shell
GITHUB_API_TOKEN=your_github_api_token
ROCKET_SECRET_KEY=your_rocket_secret_key
```

### Run
```Shell
docker-compose up --build
```
