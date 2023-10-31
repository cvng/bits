# Custom Mutations

- status: accepted
- date: 2023-10-20

## Context

SeaORM automatically generates CRUD Mutations for us; but it's rare that these will cover all our needs - and many people just disable them outright.

## Options

- <a id="1" href="https://www.graphile.org/postgraphile/custom-mutations">Custom Mutations</a>
- <a id="2" href="https://www.sea-ql.org/SeaORM/docs/seaography/seaography-intro">SeaORM Mutations</a>

## Decision

- [Custom Mutations](#1)

Custom mutations enable us to write exactly the business logic we need with access to all of our data all wrapped up in a custom function.
