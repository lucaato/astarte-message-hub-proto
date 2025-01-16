use mockall::mock;

mock! {
    pub MessageHubClient<T: 'static> {
        pub async fn attach<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<crate::MessageHubEvent>>,
            tonic::Status,
        >
        where
            R: tonic::IntoRequest<crate::Node> + 'static;

        pub async fn send<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>
        where
            R: tonic::IntoRequest<crate::AstarteMessage> + 'static;

        pub async fn detach<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>
        where
            R: tonic::IntoRequest<::pbjson_types::Empty> + 'static;

        pub async fn add_interfaces<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>
        where
            R: tonic::IntoRequest<crate::InterfacesJson> + 'static;

        pub async fn remove_interfaces<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<tonic::Response<::pbjson_types::Empty>, tonic::Status>
        where
            R: tonic::IntoRequest<crate::InterfacesName> + 'static;

        pub async fn get_properties<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<
            tonic::Response<crate::StoredProperties>,
            tonic::Status,
        >
        where
            R: tonic::IntoRequest<crate::InterfacesName> + 'static;

        pub async fn get_all_properties<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<
            tonic::Response<crate::StoredProperties>,
            tonic::Status,
        >
        where
            R: tonic::IntoRequest<crate::StoredPropertiesFilter> + 'static;

        pub async fn get_property<R>(
            &mut self,
            request: R,
        ) -> std::result::Result<tonic::Response<crate::Property>, tonic::Status>
        where
            R: tonic::IntoRequest<crate::PropertyIdentifier> + 'static;
    }
}
