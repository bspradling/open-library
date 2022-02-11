# Models 

## Works 

### Field Frequency

The existence of certain fields were found to vary by record. To understand the impacts, 
the following is a frequency distribution over 10 million records:  

| Field Name         | Existence Frequency |
|--------------------|---------------------|
| created            | 100%                |
| last_modified      | 100%                |
| latest_revision    | 100%                |
| key                | 100%                |
| type               | 100%                |
| revision           | 100%                |
| title              | 99.99%              |
| authors            | 95.23%              |
| subjects           | 58.8%               |
| covers             | 32.0%               |
| subject_places     | 24.7%               |
| first_publish_date | 18.57%              |
| id                 | 13.47%              |
| lc_classifications | 9.9%                |
| subject_people     | 7.7                 |
| subject_times      | 6.07                |
| dewey_number       | 4.8%                |
| description        | 4.4%                |
| subtitle           | 2.4%                |
| links              | 0.19%               |
| first_sentence     | 0.3%                |
| excerpts           | 0.2%                |
| cover_edition      | 0.008%              |
| location           | 0.0002%             |

This client is choosing not to support fields with less than 20% frequency until there is an explicit
reason to do so. 

### Data Quality Issues

1. There is one record in the Data Dump (`OL25303131W`) that doesn't have a title. However,
this seems to be "fixed" via the API responses. 
2. There are a number of fields that only 1 record has: 
   1. `original_languages`
   2. `table_of_contents`
   3. `number_of_editions`
   4. `notes`
   5. `notifications`
3. The `description` field has inconsistent data format.

   **Example**:

   | Identifier | Data Type | Value                                   |
   |------------|-----------|-----------------------------------------|
   | OL499789W  | object    | { "type": "/type/text", "value" "..." } |
   | OL39625W   | string    | "..."                                   |

4. The `type` within the `authors` array has an inconsistent data format

   **Example**:

   | Identifier | Data Type | Value                                   |
   |------------|-----------|-----------------------------------------|
   | OL499789W  | object    | { "type": "/type/text", "value" "..." } |
   | OL39625W   | string    | "..."                                   |
