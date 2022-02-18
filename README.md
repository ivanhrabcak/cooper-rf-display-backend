# API documentation

## Response
Definiton of the response type in Typescript
```typescript
type Response<T> = {
    status_code: number,
    response: T
}
```
All endpoints return JSON (if not specified).

#Edupage
##  `/api/edupage/substitution/<date>`
- __Method:__ GET
- __Parameters:__
    - `date` - in the `%Y-%m-%d` format (for example: `2022-02-18`)
- __Returns:__ `Response<String>` - list (divided by commas) of teachers missing on `date`
- __Example output:__
```json
{
    "response": "Chýbajúci učitelia: Denisa Bednárová , Zlatica Bindasová  (1 - 7), Iveta Holubcová , Katarína Chaloupková , Katarína Jánková  (4 - 5), Štefánia Jankovičová  (7), Ján Predáč , Branislav Rusňák , Dagmar Uríčková ",
    "status": 200
}
```

## `/api/edupage/nextlesson?<hours>&<minutes>`
- __Method__: GET
- __Parameters:__
    - `hours` and `minutes`: the time from which you want to calculate the next lesson time.
- __Returns:__ `Response<String>` - Error text, `Weekend!` or `%H:%M` time of the next lesson
- __Example output:__
```json
{
    "response": "08:15",
    "status": 200
}
```

# Dashboard
## `/api/dashboard/storage`
- __Method__: GET
- __Parameters:__
    - No parameters
- __Returns:__ `Response<u64>` - Size of the `data` folder, in bytes.
- __Example output:__
```json
{
    "response": 359919,
    "status": 200
}
```

# COOPER RF data
## `/api/data/points`
- __Method__: GET
- __Parameters:__
    - No parameters
- __Returns:__ `Response<Array<String>>` - An array with dates with data available.
- __Example output:__
```json
{
    "response": [
        "2022-02-14",
        "2022-02-12",
        "2022-02-13",
        "2022-02-18"
    ],
    "status": 200
}
```
## `/api/data/points/<date>`
- __Method__: GET
- __Parameters:__
    - `date`: Date to use when searching for data points
- __Returns:__ `Response<Map<String, Array<String>>>` - the keys of the map are ids of the stations that recorded this data. The value is an array of dates and times when data was recorded. This endpoints only returns data from one day (`date`)
- __Example output:__
```json
{
    "response": {
        "0144504602734391": [
            "2022-02-18 22:06:14",
            "2022-02-18 22:06:43",
        ],
        "0144504602734355": [
            "2022-02-18 22:06:37",
            "2022-02-18 22:07:06",
        ],
        "0144504602718778": [
            "2022-02-18 22:06:34",
            "2022-02-18 22:07:33",
        ],
        "0144504602714374": [
            "2022-02-18 22:06:37",
            "2022-02-18 22:07:06",
        ]
    },
    "status": 200
}
```