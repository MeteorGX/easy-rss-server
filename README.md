# easy-rss-bin

Build a consulting flow based on RSS information.

## Build && Install

```plain
$ cd /tmp
$ git clone https://github.com/MeteorGX/easy-rss-server.git
$ cd easy-rss-server # Go to the workspace.
$ sudo cargo build --release # Build Project
$ sudo cp target/release/easy-rss-cli /usr/bin/
```

## Register System Service

```plain
$ sudo mkdir /etc/easy-rss
$ sudo cp rss_src/* /etc/easy-rss
$ sudo cp easy-rss-cli@.service /etc/systemd/system
$ sudo cp easy-rss-cli@.timer /etc/systemd/system
$ sudo systemctl daemon-reload
```


## Start Service.

> You need to create a database. ( `use utf8mb4 charset.` )
>> Modify the database information in the jason file, where the `rss` database is used by default.

Suppose you use the `zhihu.json` configuration:

```plain
$ sudo vim /etc/easy-rss/zhihu.json # Modify the database information.
$ sudo systemctl start easy-rss-cli@zhihu.service # Test
$ sudo systemctl disable easy-rss-cli@zhihu.service # Turn off boot start.
$ sudo systemctl start easy-rss-cli@zhihu.timer
$ sudo systemctl enable easy-rss-cli@zhihu.timer
```

Ok, Enjoy!

## Other

You can access the AI interface and extract the title for information flow categorization and data analysis.This allows you to use this information to better classify.



