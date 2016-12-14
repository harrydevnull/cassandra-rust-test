# cassandra-rust-test

Testing https://github.com/AlexPikalov/cdrs driver with cassandra

1. download docker image and run
`docker run --name cassandra-1 -d -p 9042:9042 -p 9160:9160 cassandra:2.2.1`

2. cqlsh into the docker cassandra shell
   a. `docker run -it --rm --net container:cassandra-1 poklet/cassandra cqlsh`
   b.  execute the following cql scripts
   `
    CREATE KEYSPACE IF NOT EXISTS cycling WITH REPLICATION = { 'class' : 'NetworkTopologyStrategy', 'datacenter1' : 3 };

        CREATE TABLE person(
        id int PRIMARY KEY,
        name text ,
        city text,
        phone varint
        );


        INSERT INTO person (id, name, city,phone) VALUES(1,'ram', 'fll', 9848022338);
    `
3. cargo build
4. ./target/debug/cassandraaccess





 
