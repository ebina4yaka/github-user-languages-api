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

options
```
https://github-user-languages-api.herokuapp.com/user/ebina4yaka?limit=6&hide=css,vim script,PLpgSQL,makefile,shell,javascript,dockerfile
```

### Response

```
[
  {
    "name": string,
    "color": string,
    "percentage": number
  }
]
```

example

```Json
[
	{
		"name": "TypeScript",
		"color": "#2b7489",
		"percentage": 80.53
	},
	{
		"name": "Rust",
		"color": "#dea584",
		"percentage": 10.54
	},
	{
		"name": "JavaScript",
		"color": "#f1e05a",
		"percentage": 4.56
	},
	{
		"name": "Elm",
		"color": "#60B5CC",
		"percentage": 2.24
	},
	{
		"name": "CSS",
		"color": "#563d7c",
		"percentage": 1.37
	},
	{
		"name": "Dockerfile",
		"color": "#384d54",
		"percentage": 0.76
	}
]
```

### Hide individual languages

You can use `?hide=language1,language2` parameter to hide individual languages.

example

```
https://github-user-languages-api.herokuapp.com/user/ebina4yaka?hide=css,html,dockerfile
```

### Limit the number of languages

example

```
https://github-user-languages-api.herokuapp.com/user/ebina4yaka?limit=6
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
