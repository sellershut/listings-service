use sellershut_core::{
    google,
    listings::{
        mutate_listings_server::MutateListings, CreateListingRequest, CreateListingResponse,
        DeleteistingRequest, UpsertListingResponse, UpsertistingRequest,
    },
};
use tonic::{Request, Response, Status};

use super::AppState;

#[tonic::async_trait]
impl MutateListings for AppState {
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create_listing(
        &self,
        request: Request<CreateListingRequest>,
    ) -> Result<Response<CreateListingResponse>, Status> {
        todo!()
    }

    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]

    async fn upsert_listing(
        &self,
        request: Request<UpsertistingRequest>,
    ) -> Result<Response<UpsertListingResponse>, Status> {
        todo!()
    }

    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn delete_listing(
        &self,
        request: tonic::Request<DeleteistingRequest>,
    ) -> Result<Response<google::protobuf::Empty>, Status> {
        todo!()
    }
}
