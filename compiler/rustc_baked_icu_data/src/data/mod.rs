// @generated
mod fallback;
mod list;
/// This data provider was programmatically generated by [`icu_datagen`](
/// https://unicode-org.github.io/icu4x-docs/doc/icu_datagen/enum.Out.html#variant.Module).
#[non_exhaustive]
pub struct BakedDataProvider;
use ::icu_provider::prelude::*;
impl DataProvider<::icu_list::provider::AndListV1Marker> for BakedDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<::icu_list::provider::AndListV1Marker>, DataError> {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                *list::and_v1::DATA
                    .get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse())
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale
                            .with_req(::icu_list::provider::AndListV1Marker::KEY, req)
                    })?,
            ))),
        })
    }
}
impl DataProvider<::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker>
    for BakedDataProvider
{
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<
        DataResponse<
            ::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker,
        >,
        DataError,
    > {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                *fallback::supplement::co_v1::DATA.get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).ok_or_else(|| {
                    DataErrorKind::MissingLocale.with_req(::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker::KEY, req)
                })?,
            ))),
        })
    }
}
impl DataProvider<::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker>
    for BakedDataProvider
{
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<
        DataResponse<
            ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker,
        >,
        DataError,
    > {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                *fallback::likelysubtags_v1::DATA.get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).ok_or_else(|| {
                    DataErrorKind::MissingLocale.with_req(::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY, req)
                })?,
            ))),
        })
    }
}
impl DataProvider<::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker>
    for BakedDataProvider
{
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<
        DataResponse<::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker>,
        DataError,
    > {
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(zerofrom::ZeroFrom::zero_from(
                *fallback::parents_v1::DATA.get_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).ok_or_else(|| {
                    DataErrorKind::MissingLocale.with_req(::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY, req)
                })?,
            ))),
        })
    }
}