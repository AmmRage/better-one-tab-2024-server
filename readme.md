# server for [better-one-tab-2024](https://github.com/AmmRage/better-one-tab-2024)

simple server to serve the one tab extension sync data api

## Purpose

1. Serve the one tab extension sync data
2. learn rust
3. for fun

## Usage

### Config

file name: `appsettings.json`

* rotate_type: value must be one of `history_count`, `stored_time` or `total_size`
* rotate_count: integer, how many you want to keep
* rotate_time: integer, how many days you want to keep
* rotate_size: integer, how many MB in total you want to keep
* enable_region_block: boolean, enable region block
* white_region_code_list: array of string, white list of region code

```json
{
  "settings": {
    "rotate_type": "stored_time",
    "rotate_count": 10,
    "rotate_time": 1,
    "rotate_size": 21,
    "enable_region_block": true,
    "white_region_code_list": ["SG"]
  }
}
```

### Deploy with executable

eg: bind with port 9401

```
better-one-tab-2024-server 9401
```

### Deploy with docker

eg: use `/home/ubuntu/tabs/data` store data and `/home/ubuntu/tabs` store config

```
sudo docker run -it -d \
--name tabs \
-v /home/ubuntu/tabs/data:/app/data \
-v /home/ubuntu/tabs:/app/config \
-p 9401:9401 \
ammrage/tabs-server:latest
```

## To do

- [ ] add logging
- [ ] add storage abstraction layer
- [ ] add database (mongodb/redis/postgresql) support option
- [ ] add docker compose file
- [ ] add api of history CRUD