use mockall::mock;

pub type AttachStream<T> = futures::stream::Iter<std::vec::IntoIter<Result<T, tonic::Status>>>;

mock! {
    pub MessageHubClient<T: 'static> {
         pub async fn attach(
            &mut self,
            request: tonic::Request<crate::Node>,
        ) -> std::result::Result<
            tonic::Response<AttachStream<crate::MessageHubEvent>>,
            tonic::Status,
        >;

        pub async fn send(
            &mut self,
            request: tonic::Request<crate::AstarteMessage>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;

        pub async fn detach(
            &mut self,
            request: tonic::Request<::pbjson_types::Empty>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;

        pub async fn add_interfaces(
            &mut self,
            request: tonic::Request<crate::InterfacesJson>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;

        pub async fn remove_interfaces(
            &mut self,
            request: tonic::Request<crate::InterfacesName>,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>;

        pub async fn get_properties(
            &mut self,
            request: tonic::Request<crate::InterfacesName>,
        ) -> std::result::Result<
            tonic::Response<crate::StoredProperties>,
            tonic::Status,
        >;

        pub async fn get_all_properties(
            &mut self,
            request: tonic::Request<crate::StoredPropertiesFilter>,
        ) -> std::result::Result<
            tonic::Response<crate::StoredProperties>,
            tonic::Status,
        >;

        pub async fn get_property(
            &mut self,
            request: tonic::Request<crate::PropertyIdentifier>,
        ) -> std::result::Result<tonic::Response<crate::Property>, tonic::Status>;
    }

    impl<T: 'static> Clone for MessageHubClient<T> {
        fn clone(&self) -> Self;
    }
}
