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

export interface CreateShowInput {
  name: Scalars["String"];
}

export interface CreateShowPayload {
  __typename?: "CreateShowPayload";
  id?: Maybe<Scalars["ID"]>;
  show?: Maybe<Show>;
}

/** Boolean expression to compare columns of type "String". All fields are combined with logical 'AND'. */
export interface String_Comparison_Exp {
  _eq?: InputMaybe<Scalars["String"]>;
  _gt?: InputMaybe<Scalars["String"]>;
  _gte?: InputMaybe<Scalars["String"]>;
  /** does the column match the given case-insensitive pattern */
  _ilike?: InputMaybe<Scalars["String"]>;
  _in?: InputMaybe<Array<Scalars["String"]>>;
  /** does the column match the given POSIX regular expression, case insensitive */
  _iregex?: InputMaybe<Scalars["String"]>;
  _is_null?: InputMaybe<Scalars["Boolean"]>;
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

/** mutation root */
export interface Mutation_Root {
  __typename?: "mutation_root";
  create_show: CreateShowPayload;
  /** delete data from the table: "show" */
  delete_show?: Maybe<Show_Mutation_Response>;
  /** delete single row from the table: "show" */
  delete_show_by_pk?: Maybe<Show>;
  /** insert data into the table: "show" */
  insert_show?: Maybe<Show_Mutation_Response>;
  /** insert a single row into the table: "show" */
  insert_show_one?: Maybe<Show>;
  /** update data of the table: "show" */
  update_show?: Maybe<Show_Mutation_Response>;
  /** update single row of the table: "show" */
  update_show_by_pk?: Maybe<Show>;
}

/** mutation root */
export interface Mutation_RootCreate_ShowArgs {
  input: CreateShowInput;
}

/** mutation root */
export interface Mutation_RootDelete_ShowArgs {
  where: Show_Bool_Exp;
}

/** mutation root */
export interface Mutation_RootDelete_Show_By_PkArgs {
  id: Scalars["uuid"];
}

/** mutation root */
export interface Mutation_RootInsert_ShowArgs {
  objects: Array<Show_Insert_Input>;
  on_conflict?: InputMaybe<Show_On_Conflict>;
}

/** mutation root */
export interface Mutation_RootInsert_Show_OneArgs {
  object: Show_Insert_Input;
  on_conflict?: InputMaybe<Show_On_Conflict>;
}

/** mutation root */
export interface Mutation_RootUpdate_ShowArgs {
  _set?: InputMaybe<Show_Set_Input>;
  where: Show_Bool_Exp;
}

/** mutation root */
export interface Mutation_RootUpdate_Show_By_PkArgs {
  _set?: InputMaybe<Show_Set_Input>;
  pk_columns: Show_Pk_Columns_Input;
}

/** column ordering options */
export enum Order_By {
  /** in ascending order, nulls last */
  Asc = "asc",
  /** in ascending order, nulls first */
  AscNullsFirst = "asc_nulls_first",
  /** in ascending order, nulls last */
  AscNullsLast = "asc_nulls_last",
  /** in descending order, nulls first */
  Desc = "desc",
  /** in descending order, nulls first */
  DescNullsFirst = "desc_nulls_first",
  /** in descending order, nulls last */
  DescNullsLast = "desc_nulls_last",
}

export interface Query_Root {
  __typename?: "query_root";
  /** fetch data from the table: "show" */
  show: Array<Show>;
  /** fetch aggregated fields from the table: "show" */
  show_aggregate: Show_Aggregate;
  /** fetch data from the table: "show" using primary key columns */
  show_by_pk?: Maybe<Show>;
}

export interface Query_RootShowArgs {
  distinct_on?: InputMaybe<Array<Show_Select_Column>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  order_by?: InputMaybe<Array<Show_Order_By>>;
  where?: InputMaybe<Show_Bool_Exp>;
}

export interface Query_RootShow_AggregateArgs {
  distinct_on?: InputMaybe<Array<Show_Select_Column>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  order_by?: InputMaybe<Array<Show_Order_By>>;
  where?: InputMaybe<Show_Bool_Exp>;
}

export interface Query_RootShow_By_PkArgs {
  id: Scalars["uuid"];
}

/** columns and relationships of "show" */
export interface Show {
  __typename?: "show";
  id: Scalars["uuid"];
  name: Scalars["String"];
}

/** aggregated selection of "show" */
export interface Show_Aggregate {
  __typename?: "show_aggregate";
  aggregate?: Maybe<Show_Aggregate_Fields>;
  nodes: Array<Show>;
}

/** aggregate fields of "show" */
export interface Show_Aggregate_Fields {
  __typename?: "show_aggregate_fields";
  count: Scalars["Int"];
  max?: Maybe<Show_Max_Fields>;
  min?: Maybe<Show_Min_Fields>;
}

/** aggregate fields of "show" */
export interface Show_Aggregate_FieldsCountArgs {
  columns?: InputMaybe<Array<Show_Select_Column>>;
  distinct?: InputMaybe<Scalars["Boolean"]>;
}

/** Boolean expression to filter rows from the table "show". All fields are combined with a logical 'AND'. */
export interface Show_Bool_Exp {
  _and?: InputMaybe<Array<Show_Bool_Exp>>;
  _not?: InputMaybe<Show_Bool_Exp>;
  _or?: InputMaybe<Array<Show_Bool_Exp>>;
  id?: InputMaybe<Uuid_Comparison_Exp>;
  name?: InputMaybe<String_Comparison_Exp>;
}

/** unique or primary key constraints on table "show" */
export enum Show_Constraint {
  /** unique or primary key constraint */
  ShowPkey = "show_pkey",
}

/** input type for inserting data into table "show" */
export interface Show_Insert_Input {
  id?: InputMaybe<Scalars["uuid"]>;
  name?: InputMaybe<Scalars["String"]>;
}

/** aggregate max on columns */
export interface Show_Max_Fields {
  __typename?: "show_max_fields";
  id?: Maybe<Scalars["uuid"]>;
  name?: Maybe<Scalars["String"]>;
}

/** aggregate min on columns */
export interface Show_Min_Fields {
  __typename?: "show_min_fields";
  id?: Maybe<Scalars["uuid"]>;
  name?: Maybe<Scalars["String"]>;
}

/** response of any mutation on the table "show" */
export interface Show_Mutation_Response {
  __typename?: "show_mutation_response";
  /** number of rows affected by the mutation */
  affected_rows: Scalars["Int"];
  /** data from the rows affected by the mutation */
  returning: Array<Show>;
}

/** on_conflict condition type for table "show" */
export interface Show_On_Conflict {
  constraint: Show_Constraint;
  update_columns?: Array<Show_Update_Column>;
  where?: InputMaybe<Show_Bool_Exp>;
}

/** Ordering options when selecting data from "show". */
export interface Show_Order_By {
  id?: InputMaybe<Order_By>;
  name?: InputMaybe<Order_By>;
}

/** primary key columns input for table: show */
export interface Show_Pk_Columns_Input {
  id: Scalars["uuid"];
}

/** select columns of table "show" */
export enum Show_Select_Column {
  /** column name */
  Id = "id",
  /** column name */
  Name = "name",
}

/** input type for updating data in table "show" */
export interface Show_Set_Input {
  id?: InputMaybe<Scalars["uuid"]>;
  name?: InputMaybe<Scalars["String"]>;
}

/** update columns of table "show" */
export enum Show_Update_Column {
  /** column name */
  Id = "id",
  /** column name */
  Name = "name",
}

export interface Subscription_Root {
  __typename?: "subscription_root";
  /** fetch data from the table: "show" */
  show: Array<Show>;
  /** fetch aggregated fields from the table: "show" */
  show_aggregate: Show_Aggregate;
  /** fetch data from the table: "show" using primary key columns */
  show_by_pk?: Maybe<Show>;
}

export interface Subscription_RootShowArgs {
  distinct_on?: InputMaybe<Array<Show_Select_Column>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  order_by?: InputMaybe<Array<Show_Order_By>>;
  where?: InputMaybe<Show_Bool_Exp>;
}

export interface Subscription_RootShow_AggregateArgs {
  distinct_on?: InputMaybe<Array<Show_Select_Column>>;
  limit?: InputMaybe<Scalars["Int"]>;
  offset?: InputMaybe<Scalars["Int"]>;
  order_by?: InputMaybe<Array<Show_Order_By>>;
  where?: InputMaybe<Show_Bool_Exp>;
}

export interface Subscription_RootShow_By_PkArgs {
  id: Scalars["uuid"];
}

/** Boolean expression to compare columns of type "uuid". All fields are combined with logical 'AND'. */
export interface Uuid_Comparison_Exp {
  _eq?: InputMaybe<Scalars["uuid"]>;
  _gt?: InputMaybe<Scalars["uuid"]>;
  _gte?: InputMaybe<Scalars["uuid"]>;
  _in?: InputMaybe<Array<Scalars["uuid"]>>;
  _is_null?: InputMaybe<Scalars["Boolean"]>;
  _lt?: InputMaybe<Scalars["uuid"]>;
  _lte?: InputMaybe<Scalars["uuid"]>;
  _neq?: InputMaybe<Scalars["uuid"]>;
  _nin?: InputMaybe<Array<Scalars["uuid"]>>;
}

export type CreateShowMutationVariables = Exact<{
  name?: InputMaybe<Scalars["String"]>;
}>;

export type CreateShowMutation = {
  __typename?: "mutation_root";
  show?: { __typename?: "show"; id: string } | null;
};

export const CreateShowDocument = gql`
    mutation createShow($name: String) {
  show: insert_show_one(object: {name: $name}) {
    ... on show {
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
