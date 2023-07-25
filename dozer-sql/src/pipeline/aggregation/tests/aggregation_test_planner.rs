use crate::pipeline::aggregation::processor::AggregationProcessor;
use crate::pipeline::planner::projection::CommonPlanner;
use crate::pipeline::tests::utils::get_select;
use dozer_core::executor_operation::ProcessorOperation;
use dozer_core::processor_record::{ProcessorRecord, ProcessorRecordRef};
use dozer_types::types::{Field, FieldDefinition, FieldType, Schema, SourceDefinition};

#[test]
fn test_planner_with_aggregator() {
    let sql = "SELECT CONCAT(city,'/',country), CONCAT('Total: ', CAST(SUM(adults_count + children_count) AS STRING), ' people') as headcounts GROUP BY CONCAT(city,'/',country)";
    let schema = Schema::default()
        .field(
            FieldDefinition::new(
                "household_name".to_string(),
                FieldType::String,
                false,
                SourceDefinition::Table {
                    name: "households".to_string(),
                    connection: "test".to_string(),
                },
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "city".to_string(),
                FieldType::String,
                false,
                SourceDefinition::Table {
                    name: "households".to_string(),
                    connection: "test".to_string(),
                },
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "country".to_string(),
                FieldType::String,
                false,
                SourceDefinition::Table {
                    name: "households".to_string(),
                    connection: "test".to_string(),
                },
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "adults_count".to_string(),
                FieldType::Int,
                false,
                SourceDefinition::Table {
                    name: "households".to_string(),
                    connection: "test".to_string(),
                },
            ),
            false,
        )
        .field(
            FieldDefinition::new(
                "children_count".to_string(),
                FieldType::Int,
                false,
                SourceDefinition::Table {
                    name: "households".to_string(),
                    connection: "test".to_string(),
                },
            ),
            false,
        )
        .clone();

    let mut projection_planner = CommonPlanner::new(schema.clone());
    let statement = get_select(sql).unwrap();

    projection_planner.plan(*statement).unwrap();

    let mut processor = AggregationProcessor::new(
        "".to_string(),
        projection_planner.groupby,
        projection_planner.aggregation_output,
        projection_planner.projection_output,
        projection_planner.having,
        schema,
        projection_planner.post_aggregation_schema,
    )
    .unwrap();

    let mut rec = ProcessorRecord::new();
    rec.push(Field::String("John Smith".to_string()));
    rec.push(Field::String("Johor".to_string()));
    rec.push(Field::String("Malaysia".to_string()));
    rec.push(Field::Int(2));
    rec.push(Field::Int(1));
    let _r = processor
        .aggregate(ProcessorOperation::Insert {
            new: ProcessorRecordRef::new(rec),
        })
        .unwrap();

    let mut rec = ProcessorRecord::new();
    rec.push(Field::String("Todd Enton".to_string()));
    rec.push(Field::String("Johor".to_string()));
    rec.push(Field::String("Malaysia".to_string()));
    rec.push(Field::Int(2));
    rec.push(Field::Int(1));
    let _r = processor
        .aggregate(ProcessorOperation::Insert {
            new: ProcessorRecordRef::new(rec),
        })
        .unwrap();
}
