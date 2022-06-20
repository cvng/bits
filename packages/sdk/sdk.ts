import { gql, GraphQLClient } from "graphql_request/mod.ts";
import * as Dom from "graphql_request/src/types.dom.ts";
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> =
  & Omit<T, K>
  & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> =
  & Omit<T, K>
  & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export interface Scalars {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  uuid: string;
}

/** columns and relationships of "show" */
export interface Show {
  __typename?: "Show";
  id: Scalars["uuid"];
  name: Scalars["String"];
  namingConvention?: Maybe<Scalars["String"]>;
}

/** aggregated selection of "show" */
export interface ShowAggregate {
  __typename?: "ShowAggregate";
  aggregate?: Maybe<ShowAggregateFields>;
  nodes: Array<Show>;
}

/** aggregate fields of "show" */
export interface ShowAggregateFields {
  __typename?: "ShowAggregateFields";
  count: Scalars["Int"];
  max?: Maybe<ShowMaxFields>;
  min?: Maybe<ShowMinFields>;
}

/** aggregate fields of "show" */
export interface ShowAggregateFieldsCountArgs {
  columns?: InputMaybe<Array<ShowSelectColumn>>;
  distinct?: InputMaybe<Scalars["Boolean"]>;
}

/** Boolean expression to filter rows from the table "show". All fields are combined with a logical 'AND'. */
export interface ShowBoolExp {
  _and?: InputMaybe<Array<ShowBoolExp>>;
  _not?: InputMaybe<ShowBoolExp>;
  _or?: InputMaybe<Array<ShowBoolExp>>;
  id?: InputMaybe<UuidComparisonExp>;
  name?: InputMaybe<StringComparisonExp>;
  namingConvention?: InputMaybe<StringComparisonExp>;
}

/** unique or primary key constraints on table "show" */
export enum ShowConstraint {
  /** unique or primary key constraint on columns "id" */
  ShowPkey = "show_pkey",
}

/** input type for inserting data into table "show" */
export interface ShowInsertInput {
  id?: InputMaybe<Scalars["uuid"]>;
  name?: InputMaybe<Scalars["String"]>;
  namingConvention?: InputMaybe<Scalars["String"]>;
}

/** aggregate max on columns */
export interface ShowMaxFields {
  __typename?: "ShowMaxFields";
  id?: Maybe<Scalars["uuid"]>;
  name?: Maybe<Scalars["String"]>;
  namingConvention?: Maybe<Scalars["String"]>;
}

/** aggregate min on columns */
export interface ShowMinFields {
  __typename?: "ShowMinFields";
  id?: Maybe<Scalars["uuid"]>;
  name?: Maybe<Scalars["String"]>;
  namingConvention?: Maybe<Scalars["String"]>;
}

/** response of any mutation on the table "show" */
export interface ShowMutationResponse {
  __typename?: "ShowMutationResponse";
  /** number of rows affected by the mutation */
  affected_rows: Scalars["Int"];
  /** data from the rows affected by the mutation */
  returning: Array<Show>;
}

/** on_conflict condition type for table "show" */
export interface ShowOnConflict {
  constraint: ShowConstraint;
  update_columns?: Array<ShowUpdateColumn>;
  where?: InputMaybe<ShowBoolExp>;
}

/** Ordering options when selecting data from "show". */
export interface ShowOrderBy {
  id?: InputMaybe<OrderBy>;
  name?: InputMaybe<OrderBy>;
  namingConvention?: InputMaybe<OrderBy>;
}

/** primary key columns input for table: show */
export interface ShowPkColumnsInput {
  id: Scalars["uuid"];
}

/** select columns of table "show" */
export enum ShowSelectColumn {
  /** column name */
  Id = "id",
  /** column name */
  Name = "name",
  /** column name */
  NamingConvention = "namingConvention",
}

/** input type for updating data in table "show" */
export interface ShowSetInput {
  id?: InputMaybe<Scalars["uuid"]>;
  name?: InputMaybe<Scalars["String"]>;
  namingConvention?: InputMaybe<Scalars["String"]>;
}

/** update columns of table "show" */
export enum ShowUpdateColumn {
  /** column name */
  Id = "id",
  /** column name */
  Name = "name",
  /** column name */
  NamingConvention = "namingConvention",
}

/** Boolean expression to compare columns of type "String". All fields are combined with logical 'AND'. */
export interface StringComparisonExp {
  _eq?: InputMaybe<Scalars["String"]>;
  _gt?: InputMaybe<Scalars["String"]>;
  _gte?: InputMaybe<Scalars["String"]>;
  /** does the column match the given case-insensitive pattern */
  _ilike?: InputMaybe<Scalars["String"]>;
  _in?: InputMaybe<Array<Scalars["String"]>>;
  /** does the column match the given POSIX regular expression, case insensitive */
  _iregex?: InputMaybe<Scalars["String"]>;
  _isNull?: InputMaybe<Scalars["Boolean"]>;
  /** does the column match the given pattern */
  _like?: InputMaybe<Scalars["String"]>;
  _lt?: InputMaybe<Scalars["String"]>;
  _lte?: InputMaybe<Scalars["String"]>;
  _neq?: InputMaybe<Scalars["String"]>;
  /** does the column NOT match the given case-insensitive pattern */
  _nilike?: InputMaybe<Scalars["String"]>;
  _nin?: InputMaybe<Array<Scalars["String"]>>;
  /** does the column NOT match the given POSIX regular expression, case insensitive */
  _niregex?: InputMaybe<Scalars["String"]>;
  /** does the column NOT match the given pattern */
  _nlike?: InputMaybe<Scalars["String"]>;
  /** does the column NOT match the given POSIX regular expression, case sensitive */
  _nregex?: InputMaybe<Scalars["String"]>;
  /** does the column NOT match the given SQL regular expression */
  _nsimilar?: InputMaybe<Scalars["String"]>;
  /** does the column match the given POSIX regular expression, case sensitive */
  _regex?: InputMaybe<Scalars["String"]>;
  /** does the column match the given SQL regular expression */
  _similar?: InputMaybe<Scalars["String"]>;
}

/** Boolean expression to compare columns of type "uuid". All fields are combined with logical 'AND'. */
export interface UuidComparisonExp {
  _cast?: InputMaybe<Uuid_Cast_Exp>;
  _eq?: InputMaybe<Scalars["uuid"]>;
  _gt?: InputMaybe<Scalars["uuid"]>;
  _gte?: InputMaybe<Scalars["uuid"]>;
  _in?: InputMaybe<Array<Scalars["uuid"]>>;
  _isNull?: InputMaybe<Scalars["Boolean"]>;
  _lt?: InputMaybe<Scalars["uuid"]>;
  _lte?: InputMaybe<Scalars["uuid"]>;
  _neq?: InputMaybe<Scalars["uuid"]>;
  _nin?: InputMaybe<Array<Scalars["uuid"]>>;
}

/** mutation root */
export interface Mutation_Root {
  __typename?: "mutation_root";
  /** delete data from the table: "show" */
  deleteShow?: Maybe<ShowMutationResponse>;
  /** delete single row from the table: "show" */
  deleteShowByPk?: Maybe<Show>;
  /** insert data into the table: "show" */
  insertShow?: Maybe<ShowMutationResponse>;
  /** insert a single row into the table: "show" */
  insertShowOne?: Maybe<Show>;
  /** update data of the table: "show" */
  updateShow?: Maybe<ShowMutationResponse>;
  /** update single row of the table: "show" */
  updateShowByPk?: Maybe<Show>;
}

/** mutation root */
export interface Mutation_RootDeleteShowArgs {
  where: ShowBoolExp;
}

/** mutation root */
export interface Mutation_RootDeleteShowByPkArgs {
  id: Scalars["uuid"];
}

/** mutation root */
export interface Mutation_RootInsertShowArgs {
  objects: Array<ShowInsertInput>;
  onConflict?: InputMaybe<ShowOnConflict>;
}

/** mutation root */
export interface Mutation_RootInsertShowOneArgs {
  object: ShowInsertInput;
  onConflict?: InputMaybe<ShowOnConflict>;
}

/** mutation root */
export interface Mutation_RootUpdateShowArgs {
  _set?: InputMaybe<ShowSetInput>;
  where: ShowBoolExp;
}

/** mutation root */
export interface Mutation_RootUpdateShowByPkArgs {
  _set?: InputMaybe<ShowSetInput>;
  pk_columns: ShowPkColumnsInput;
}

/** column ordering options */
export enum OrderBy {
  /** in ascending order, nulls last */
  Asc = "asc",
  /** in ascending order, nulls first */
  AscNullsFirst = "ascNullsFirst",
  /** in ascending order, nulls last */
  AscNullsLast = "ascNullsLast",
  /** in descending order, nulls first */
  Desc = "desc",
  /** in descending order, nulls first */
  DescNullsFirst = "descNullsFirst",
  /** in descending order, nulls last */
  DescNullsLast = "descNullsLast",
}

export interface Query_Root {
  __typename?: "query_root";
  /** fetch aggregated fields from the table: "show" */
  showAggregate: ShowAggregate;
  /** fetch data from the table: "show" using primary key columns */
  showByPk?: Maybe<Show>;
  /** fetch data from the table: "show" */
  shows: Array<Show>;
}

export interface Query_RootShowAggregateArgs {
  distinctOn?: InputMaybe<Array<ShowSelectColumn>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  orderBy?: InputMaybe<Array<ShowOrderBy>>;
  where?: InputMaybe<ShowBoolExp>;
}

export interface Query_RootShowByPkArgs {
  id: Scalars["uuid"];
}

export interface Query_RootShowsArgs {
  distinctOn?: InputMaybe<Array<ShowSelectColumn>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  orderBy?: InputMaybe<Array<ShowOrderBy>>;
  where?: InputMaybe<ShowBoolExp>;
}

export interface Subscription_Root {
  __typename?: "subscription_root";
  /** fetch aggregated fields from the table: "show" */
  showAggregate: ShowAggregate;
  /** fetch data from the table: "show" using primary key columns */
  showByPk?: Maybe<Show>;
  /** fetch data from the table: "show" */
  shows: Array<Show>;
}

export interface Subscription_RootShowAggregateArgs {
  distinctOn?: InputMaybe<Array<ShowSelectColumn>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  orderBy?: InputMaybe<Array<ShowOrderBy>>;
  where?: InputMaybe<ShowBoolExp>;
}

export interface Subscription_RootShowByPkArgs {
  id: Scalars["uuid"];
}

export interface Subscription_RootShowsArgs {
  distinctOn?: InputMaybe<Array<ShowSelectColumn>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  orderBy?: InputMaybe<Array<ShowOrderBy>>;
  where?: InputMaybe<ShowBoolExp>;
}

export interface Uuid_Cast_Exp {
  String?: InputMaybe<StringComparisonExp>;
}

export type CreateShowMutationVariables = Exact<{
  name?: InputMaybe<Scalars["String"]>;
}>;

export type CreateShowMutation = {
  __typename?: "mutation_root";
  show?: { __typename?: "Show"; id: string } | null;
};

export const CreateShowDocument = gql`
    mutation createShow($name: String) {
  show: insertShowOne(object: {name: $name}) {
    ... on Show {
      id
    }
  }
}
    `;

export type SdkFunctionWrapper = <T>(
  action: (requestHeaders?: Record<string, string>) => Promise<T>,
  operationName: string,
  operationType?: string,
) => Promise<T>;

const defaultWrapper: SdkFunctionWrapper = (
  action,
  _operationName,
  _operationType,
) => action();

export function getSdk(
  client: GraphQLClient,
  withWrapper: SdkFunctionWrapper = defaultWrapper,
) {
  return {
    createShow(
      variables?: CreateShowMutationVariables,
      requestHeaders?: Dom.RequestInit["headers"],
    ): Promise<CreateShowMutation> {
      return withWrapper(
        (wrappedRequestHeaders) =>
          client.request<CreateShowMutation>(CreateShowDocument, variables, {
            ...requestHeaders,
            ...wrappedRequestHeaders,
          }),
        "createShow",
        "mutation",
      );
    },
  };
}
export type Sdk = ReturnType<typeof getSdk>;
