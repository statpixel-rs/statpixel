use std::marker::PhantomData;

use skia_safe::Data;

#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! include_image {
	($path: expr) => {
		once_cell::sync::Lazy::new(|| $crate::from_bytes(include_bytes!($path)).unwrap())
	};
}

#[derive(Debug, Clone)]
pub struct Image<'i> {
	image: skia_safe::Image,
	phantom: PhantomData<&'i ()>,
}

impl<'i> Image<'i> {
	#[must_use]
	#[inline]
	pub fn image(&'i self) -> &'i skia_safe::Image {
		&self.image
	}
}

#[must_use]
#[inline]
pub fn from_bytes(bytes: &[u8]) -> Option<Image> {
	skia_safe::Image::from_encoded(unsafe { Data::new_bytes(bytes) }).map(|image| Image {
		image,
		phantom: PhantomData,
	})
}

#[must_use]
#[inline]
pub fn from_bytes_copy(bytes: &[u8]) -> Option<Image<'static>> {
	skia_safe::Image::from_encoded(Data::new_copy(bytes)).map(|image| Image {
		image,
		phantom: PhantomData,
	})
}
