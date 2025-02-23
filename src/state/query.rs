use sellershut_core::{
    common::pagination,
    listings::{
        query_listings_server::QueryListings, Connection, GetListingByApIdRequest,
        GetListingByApIdResponse, GetListingsByCategoryApIdRequest, GetListingsByOwnerApIdRequest,
    },
};
use tonic::{Request, Response, Status};

use super::AppState;

#[tonic::async_trait]
impl QueryListings for AppState {
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn listings(
        &self,
        request: Request<pagination::Cursor>,
    ) -> Result<Response<Connection>, Status> {
        todo!()
    }

    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn listings_by_ap_id(
        &self,
        request: Request<GetListingByApIdRequest>,
    ) -> Result<Response<GetListingByApIdResponse>, Status> {
        todo!()
    }

    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn listings_by_owner_ap_id(
        &self,
        request: Request<GetListingsByOwnerApIdRequest>,
    ) -> Result<Response<Connection>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn listings_by_category_ap_id(
        &self,
        request: Request<GetListingsByCategoryApIdRequest>,
    ) -> Result<Response<Connection>, tonic::Status> {
        todo!()
    }
}
