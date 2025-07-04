// Generated by `wit-bindgen` 0.41.0. DO NOT EDIT!
// Options used:
#[allow(dead_code, clippy::all)]
pub mod local {
  pub mod val {

    #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
    pub mod types {
      #[used]
      #[doc(hidden)]
      static __FORCE_SECTION_REF: fn() =
      super::super::super::__link_custom_section_describing_imports;
      
      use super::super::super::_rt;
      /// 所有类型定义必须放在 interface 中
      pub type Blob = _rt::Vec::<u8>;
      #[derive(Clone)]
      pub struct Decimal {
        pub prec: u32,
        pub scale: i32,
        pub value: _rt::String,
      }
      impl ::core::fmt::Debug for Decimal {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Decimal").field("prec", &self.prec).field("scale", &self.scale).field("value", &self.value).finish()
        }
      }
      #[repr(C)]
      #[derive(Clone, Copy)]
      pub struct Date {
        pub days_since_epoch: i32,
      }
      impl ::core::fmt::Debug for Date {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Date").field("days-since-epoch", &self.days_since_epoch).finish()
        }
      }
      #[repr(C)]
      #[derive(Clone, Copy)]
      pub struct Datetime {
        pub milliseconds_since_epoch: i64,
      }
      impl ::core::fmt::Debug for Datetime {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Datetime").field("milliseconds-since-epoch", &self.milliseconds_since_epoch).finish()
        }
      }
      #[repr(C)]
      #[derive(Clone, Copy)]
      pub struct Time {
        pub nanoseconds_since_midnight: i64,
      }
      impl ::core::fmt::Debug for Time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("Time").field("nanoseconds-since-midnight", &self.nanoseconds_since_midnight).finish()
        }
      }
      #[derive(Clone)]
      pub enum Val {
        Null,
        Blob(Blob),
        Boolen(bool),
        Str(_rt::String),
        Float(f64),
        Int(i64),
        Decimal(Decimal),
        Date(Date),
        Datetime(Datetime),
        Time(Time),
        DatetimeIso(_rt::String),
        DurationIso(_rt::String),
        Error(_rt::String),
      }
      impl ::core::fmt::Debug for Val {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self {
            Val::Null => {
              f.debug_tuple("Val::Null").finish()
            }
            Val::Blob(e) => {
              f.debug_tuple("Val::Blob").field(e).finish()
            }
            Val::Boolen(e) => {
              f.debug_tuple("Val::Boolen").field(e).finish()
            }
            Val::Str(e) => {
              f.debug_tuple("Val::Str").field(e).finish()
            }
            Val::Float(e) => {
              f.debug_tuple("Val::Float").field(e).finish()
            }
            Val::Int(e) => {
              f.debug_tuple("Val::Int").field(e).finish()
            }
            Val::Decimal(e) => {
              f.debug_tuple("Val::Decimal").field(e).finish()
            }
            Val::Date(e) => {
              f.debug_tuple("Val::Date").field(e).finish()
            }
            Val::Datetime(e) => {
              f.debug_tuple("Val::Datetime").field(e).finish()
            }
            Val::Time(e) => {
              f.debug_tuple("Val::Time").field(e).finish()
            }
            Val::DatetimeIso(e) => {
              f.debug_tuple("Val::DatetimeIso").field(e).finish()
            }
            Val::DurationIso(e) => {
              f.debug_tuple("Val::DurationIso").field(e).finish()
            }
            Val::Error(e) => {
              f.debug_tuple("Val::Error").field(e).finish()
            }
          }
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn get_map() -> _rt::Vec::<(_rt::String,Val,)>{
        unsafe {

          #[cfg_attr(target_pointer_width="64", repr(align(8)))]
          #[cfg_attr(target_pointer_width="32", repr(align(4)))]
          struct RetArea([::core::mem::MaybeUninit::<u8>; 2*::core::mem::size_of::<*const u8>()]);
          let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2*::core::mem::size_of::<*const u8>()]);
          let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "local:val/types@1.0.0")]
          unsafe extern "C" {
            #[link_name = "get-map"]
            fn wit_import1(_: *mut u8, );
          }

          #[cfg(not(target_arch = "wasm32"))]
          unsafe extern "C" fn wit_import1(_: *mut u8, ){ unreachable!() }
          unsafe { wit_import1(ptr0) };
          let l2 = *ptr0.add(0).cast::<*mut u8>();
          let l3 = *ptr0.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
          let base35 = l2;
          let len35 = l3;
          let mut result35 = _rt::Vec::with_capacity(len35);
          for i in 0..len35 {
            let base = base35.add(i * (16+4*::core::mem::size_of::<*const u8>()));
            let e35 = {
              let l4 = *base.add(0).cast::<*mut u8>();
              let l5 = *base.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
              let len6 = l5;
              let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
              let l7 = i32::from(*base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>());
              let v34 = match l7 {
                0 => {
                  Val::Null
                }
                1 => {
                  let e34 = {
                    let l8 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l9 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len10 = l9;

                    _rt::Vec::from_raw_parts(l8.cast(), len10, len10)
                  };
                  Val::Blob(e34)
                }
                2 => {
                  let e34 = {
                    let l11 = i32::from(*base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<u8>());

                    _rt::bool_lift(l11 as u8)
                  };
                  Val::Boolen(e34)
                }
                3 => {
                  let e34 = {
                    let l12 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l13 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len14 = l13;
                    let bytes14 = _rt::Vec::from_raw_parts(l12.cast(), len14, len14);

                    _rt::string_lift(bytes14)
                  };
                  Val::Str(e34)
                }
                4 => {
                  let e34 = {
                    let l15 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<f64>();

                    l15
                  };
                  Val::Float(e34)
                }
                5 => {
                  let e34 = {
                    let l16 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>();

                    l16
                  };
                  Val::Int(e34)
                }
                6 => {
                  let e34 = {
                    let l17 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i32>();
                    let l18 = *base.add(12+2*::core::mem::size_of::<*const u8>()).cast::<i32>();
                    let l19 = *base.add(16+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l20 = *base.add(16+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len21 = l20;
                    let bytes21 = _rt::Vec::from_raw_parts(l19.cast(), len21, len21);

                    Decimal{
                      prec: l17 as u32,
                      scale: l18,
                      value: _rt::string_lift(bytes21),
                    }
                  };
                  Val::Decimal(e34)
                }
                7 => {
                  let e34 = {
                    let l22 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i32>();

                    Date{
                      days_since_epoch: l22,
                    }
                  };
                  Val::Date(e34)
                }
                8 => {
                  let e34 = {
                    let l23 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>();

                    Datetime{
                      milliseconds_since_epoch: l23,
                    }
                  };
                  Val::Datetime(e34)
                }
                9 => {
                  let e34 = {
                    let l24 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>();

                    Time{
                      nanoseconds_since_midnight: l24,
                    }
                  };
                  Val::Time(e34)
                }
                10 => {
                  let e34 = {
                    let l25 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l26 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len27 = l26;
                    let bytes27 = _rt::Vec::from_raw_parts(l25.cast(), len27, len27);

                    _rt::string_lift(bytes27)
                  };
                  Val::DatetimeIso(e34)
                }
                11 => {
                  let e34 = {
                    let l28 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l29 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len30 = l29;
                    let bytes30 = _rt::Vec::from_raw_parts(l28.cast(), len30, len30);

                    _rt::string_lift(bytes30)
                  };
                  Val::DurationIso(e34)
                }
                n => {
                  debug_assert_eq!(n, 12, "invalid enum discriminant");
                  let e34 = {
                    let l31 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                    let l32 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                    let len33 = l32;
                    let bytes33 = _rt::Vec::from_raw_parts(l31.cast(), len33, len33);

                    _rt::string_lift(bytes33)
                  };
                  Val::Error(e34)
                }
              };

              (_rt::string_lift(bytes6), v34)
            };
            result35.push(e35);
          }
          _rt::cabi_dealloc(base35, len35 * (16+4*::core::mem::size_of::<*const u8>()), 8);
          let result36 = result35;
          result36
        }
      }

    }

  }
}
#[allow(dead_code, clippy::all)]
pub mod exports {
  pub mod local {
    pub mod val {

      #[allow(dead_code, async_fn_in_trait, unused_imports, clippy::all)]
      pub mod types {
        #[used]
        #[doc(hidden)]
        static __FORCE_SECTION_REF: fn() =
        super::super::super::super::__link_custom_section_describing_imports;
        
        use super::super::super::super::_rt;
        /// 所有类型定义必须放在 interface 中
        pub type Blob = _rt::Vec::<u8>;
        #[derive(Clone)]
        pub struct Decimal {
          pub prec: u32,
          pub scale: i32,
          pub value: _rt::String,
        }
        impl ::core::fmt::Debug for Decimal {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Decimal").field("prec", &self.prec).field("scale", &self.scale).field("value", &self.value).finish()
          }
        }
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct Date {
          pub days_since_epoch: i32,
        }
        impl ::core::fmt::Debug for Date {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Date").field("days-since-epoch", &self.days_since_epoch).finish()
          }
        }
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct Datetime {
          pub milliseconds_since_epoch: i64,
        }
        impl ::core::fmt::Debug for Datetime {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Datetime").field("milliseconds-since-epoch", &self.milliseconds_since_epoch).finish()
          }
        }
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct Time {
          pub nanoseconds_since_midnight: i64,
        }
        impl ::core::fmt::Debug for Time {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Time").field("nanoseconds-since-midnight", &self.nanoseconds_since_midnight).finish()
          }
        }
        #[derive(Clone)]
        pub enum Val {
          Null,
          Blob(Blob),
          Boolen(bool),
          Str(_rt::String),
          Float(f64),
          Int(i64),
          Decimal(Decimal),
          Date(Date),
          Datetime(Datetime),
          Time(Time),
          DatetimeIso(_rt::String),
          DurationIso(_rt::String),
          Error(_rt::String),
        }
        impl ::core::fmt::Debug for Val {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              Val::Null => {
                f.debug_tuple("Val::Null").finish()
              }
              Val::Blob(e) => {
                f.debug_tuple("Val::Blob").field(e).finish()
              }
              Val::Boolen(e) => {
                f.debug_tuple("Val::Boolen").field(e).finish()
              }
              Val::Str(e) => {
                f.debug_tuple("Val::Str").field(e).finish()
              }
              Val::Float(e) => {
                f.debug_tuple("Val::Float").field(e).finish()
              }
              Val::Int(e) => {
                f.debug_tuple("Val::Int").field(e).finish()
              }
              Val::Decimal(e) => {
                f.debug_tuple("Val::Decimal").field(e).finish()
              }
              Val::Date(e) => {
                f.debug_tuple("Val::Date").field(e).finish()
              }
              Val::Datetime(e) => {
                f.debug_tuple("Val::Datetime").field(e).finish()
              }
              Val::Time(e) => {
                f.debug_tuple("Val::Time").field(e).finish()
              }
              Val::DatetimeIso(e) => {
                f.debug_tuple("Val::DatetimeIso").field(e).finish()
              }
              Val::DurationIso(e) => {
                f.debug_tuple("Val::DurationIso").field(e).finish()
              }
              Val::Error(e) => {
                f.debug_tuple("Val::Error").field(e).finish()
              }
            }
          }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub unsafe fn _export_get_map_cabi<T: Guest>() -> *mut u8 {#[cfg(target_arch="wasm32")]
        _rt::run_ctors_once();let result0 = T::get_map();
        let ptr1 = (&raw mut _RET_AREA.0).cast::<u8>();
        let vec14 = result0;
        let len14 = vec14.len();
        let layout14 = _rt::alloc::Layout::from_size_align_unchecked(vec14.len() * (16+4*::core::mem::size_of::<*const u8>()), 8);
        let result14 = if layout14.size() != 0 {
          let ptr = _rt::alloc::alloc(layout14).cast::<u8>();
          if ptr.is_null()
          {
            _rt::alloc::handle_alloc_error(layout14);
          }
          ptr
        }else {
          ::core::ptr::null_mut()
        };
        for (i, e) in vec14.into_iter().enumerate() {
          let base = result14.add(i * (16+4*::core::mem::size_of::<*const u8>()));
          {
            let (t2_0, t2_1, ) = e;
            let vec3 = (t2_0.into_bytes()).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *base.add(::core::mem::size_of::<*const u8>()).cast::<usize>() = len3;
            *base.add(0).cast::<*mut u8>() = ptr3.cast_mut();
            match t2_1 {
              Val::Null=> {
                {
                  *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (0i32) as u8;
                }
              }
              Val::Blob(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (1i32) as u8;
                let vec4 = (e).into_boxed_slice();
                let ptr4 = vec4.as_ptr().cast::<u8>();
                let len4 = vec4.len();
                ::core::mem::forget(vec4);
                *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len4;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr4.cast_mut();
              },
              Val::Boolen(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (2i32) as u8;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (match e { true => 1, false => 0 }) as u8;
              },
              Val::Str(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (3i32) as u8;
                let vec5 = (e.into_bytes()).into_boxed_slice();
                let ptr5 = vec5.as_ptr().cast::<u8>();
                let len5 = vec5.len();
                ::core::mem::forget(vec5);
                *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len5;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr5.cast_mut();
              },
              Val::Float(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (4i32) as u8;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<f64>() = _rt::as_f64(e);
              },
              Val::Int(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (5i32) as u8;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>() = _rt::as_i64(e);
              },
              Val::Decimal(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (6i32) as u8;
                let Decimal{ prec:prec6, scale:scale6, value:value6, } = e;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i32>() = _rt::as_i32(prec6);
                *base.add(12+2*::core::mem::size_of::<*const u8>()).cast::<i32>() = _rt::as_i32(scale6);
                let vec7 = (value6.into_bytes()).into_boxed_slice();
                let ptr7 = vec7.as_ptr().cast::<u8>();
                let len7 = vec7.len();
                ::core::mem::forget(vec7);
                *base.add(16+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len7;
                *base.add(16+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr7.cast_mut();
              },
              Val::Date(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (7i32) as u8;
                let Date{ days_since_epoch:days_since_epoch8, } = e;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i32>() = _rt::as_i32(days_since_epoch8);
              },
              Val::Datetime(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (8i32) as u8;
                let Datetime{ milliseconds_since_epoch:milliseconds_since_epoch9, } = e;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>() = _rt::as_i64(milliseconds_since_epoch9);
              },
              Val::Time(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (9i32) as u8;
                let Time{ nanoseconds_since_midnight:nanoseconds_since_midnight10, } = e;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<i64>() = _rt::as_i64(nanoseconds_since_midnight10);
              },
              Val::DatetimeIso(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (10i32) as u8;
                let vec11 = (e.into_bytes()).into_boxed_slice();
                let ptr11 = vec11.as_ptr().cast::<u8>();
                let len11 = vec11.len();
                ::core::mem::forget(vec11);
                *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len11;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr11.cast_mut();
              },
              Val::DurationIso(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (11i32) as u8;
                let vec12 = (e.into_bytes()).into_boxed_slice();
                let ptr12 = vec12.as_ptr().cast::<u8>();
                let len12 = vec12.len();
                ::core::mem::forget(vec12);
                *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len12;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr12.cast_mut();
              },
              Val::Error(e) => {
                *base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>() = (12i32) as u8;
                let vec13 = (e.into_bytes()).into_boxed_slice();
                let ptr13 = vec13.as_ptr().cast::<u8>();
                let len13 = vec13.len();
                ::core::mem::forget(vec13);
                *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>() = len13;
                *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>() = ptr13.cast_mut();
              },
            }
          }
        }
        *ptr1.add(::core::mem::size_of::<*const u8>()).cast::<usize>() = len14;
        *ptr1.add(0).cast::<*mut u8>() = result14;
        ptr1
      }
      #[doc(hidden)]
      #[allow(non_snake_case)]
      pub unsafe fn __post_return_get_map<T: Guest>(arg0: *mut u8,) {
        let l0 = *arg0.add(0).cast::<*mut u8>();
        let l1 = *arg0.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
        let base18 = l0;
        let len18 = l1;
        for i in 0..len18 {
          let base = base18.add(i * (16+4*::core::mem::size_of::<*const u8>()));
          {
            let l2 = *base.add(0).cast::<*mut u8>();
            let l3 = *base.add(::core::mem::size_of::<*const u8>()).cast::<usize>();
            _rt::cabi_dealloc(l2, l3, 1);
            let l4 = i32::from(*base.add(2*::core::mem::size_of::<*const u8>()).cast::<u8>());
            match l4 {
              0 => (),
              1 => {
                let l5 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l6 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                let base7 = l5;
                let len7 = l6;
                _rt::cabi_dealloc(base7, len7 * 1, 1);
              },
              2 => (),
              3 => {
                let l8 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l9 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                _rt::cabi_dealloc(l8, l9, 1);
              },
              4 => (),
              5 => (),
              6 => {
                let l10 = *base.add(16+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l11 = *base.add(16+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                _rt::cabi_dealloc(l10, l11, 1);
              },
              7 => (),
              8 => (),
              9 => (),
              10 => {
                let l12 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l13 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                _rt::cabi_dealloc(l12, l13, 1);
              },
              11 => {
                let l14 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l15 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                _rt::cabi_dealloc(l14, l15, 1);
              },
              _ => {
                let l16 = *base.add(8+2*::core::mem::size_of::<*const u8>()).cast::<*mut u8>();
                let l17 = *base.add(8+3*::core::mem::size_of::<*const u8>()).cast::<usize>();
                _rt::cabi_dealloc(l16, l17, 1);
              },
            }
          }
        }
        _rt::cabi_dealloc(base18, len18 * (16+4*::core::mem::size_of::<*const u8>()), 8);
      }
      pub trait Guest {
        fn get_map() -> _rt::Vec::<(_rt::String,Val,)>;
      }
      #[doc(hidden)]

      macro_rules! __export_local_val_types_1_0_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[unsafe(export_name = "local:val/types@1.0.0#get-map")]
          unsafe extern "C" fn export_get_map() -> *mut u8 {
            unsafe { $($path_to_types)*::_export_get_map_cabi::<$ty>() }
          }
          #[unsafe(export_name = "cabi_post_local:val/types@1.0.0#get-map")]
          unsafe extern "C" fn _post_return_get_map(arg0: *mut u8,) {
            unsafe { $($path_to_types)*::__post_return_get_map::<$ty>(arg0) }
          }
        };);
      }
      #[doc(hidden)]
      pub(crate) use __export_local_val_types_1_0_0_cabi;

      #[cfg_attr(target_pointer_width="64", repr(align(8)))]
      #[cfg_attr(target_pointer_width="32", repr(align(4)))]
      struct _RetArea([::core::mem::MaybeUninit::<u8>; 2*::core::mem::size_of::<*const u8>()]);
      static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 2*::core::mem::size_of::<*const u8>()]);

    }

  }
}
}
mod _rt {
  #![allow(dead_code, clippy::all)]
  pub use alloc_crate::vec::Vec;
  pub use alloc_crate::string::String;
  pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
      String::from_utf8(bytes).unwrap()
    } else {
      String::from_utf8_unchecked(bytes)
    }
  }
  pub unsafe fn bool_lift(val: u8) -> bool {
    if cfg!(debug_assertions) {
      match val {
        0 => false,
        1 => true,
        _ => panic!("invalid bool discriminant"),
      }
    } else {
      val != 0
    }
  }
  pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if size == 0 {
      return;
    }
    let layout = alloc::Layout::from_size_align_unchecked(size, align);
    alloc::dealloc(ptr, layout);
  }
  
  #[cfg(target_arch = "wasm32")]
  pub fn run_ctors_once() {
    wit_bindgen::rt::run_ctors_once();
  }
  
  pub fn as_f64<T: AsF64>(t: T) -> f64 {
    t.as_f64()
  }

  pub trait AsF64 {
    fn as_f64(self) -> f64;
  }

  impl<'a, T: Copy + AsF64> AsF64 for &'a T {
    fn as_f64(self) -> f64 {
      (*self).as_f64()
    }
  }
  
  impl AsF64 for f64 {
    #[inline]
    fn as_f64(self) -> f64 {
      self as f64
    }
  }
  
  pub fn as_i64<T: AsI64>(t: T) -> i64 {
    t.as_i64()
  }

  pub trait AsI64 {
    fn as_i64(self) -> i64;
  }

  impl<'a, T: Copy + AsI64> AsI64 for &'a T {
    fn as_i64(self) -> i64 {
      (*self).as_i64()
    }
  }
  
  impl AsI64 for i64 {
    #[inline]
    fn as_i64(self) -> i64 {
      self as i64
    }
  }
  
  impl AsI64 for u64 {
    #[inline]
    fn as_i64(self) -> i64 {
      self as i64
    }
  }
  
  pub fn as_i32<T: AsI32>(t: T) -> i32 {
    t.as_i32()
  }

  pub trait AsI32 {
    fn as_i32(self) -> i32;
  }

  impl<'a, T: Copy + AsI32> AsI32 for &'a T {
    fn as_i32(self) -> i32 {
      (*self).as_i32()
    }
  }
  
  impl AsI32 for i32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u32 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u16 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for i8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for u8 {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for char {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  
  impl AsI32 for usize {
    #[inline]
    fn as_i32(self) -> i32 {
      self as i32
    }
  }
  pub use alloc_crate::alloc;
  extern crate alloc as alloc_crate;
}

/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_val_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::local::val::types::__export_local_val_types_1_0_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::local::val::types);
  )
}
#[doc(inline)]
pub(crate) use __export_val_impl as export;

#[cfg(target_arch = "wasm32")]
#[unsafe(link_section = "component-type:wit-bindgen:0.41.0:local:val@1.0.0:val:encoded world")]
#[doc(hidden)]
#[allow(clippy::octal_escapes)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 885] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xfb\x05\x01A\x02\x01\
A\x06\x01B\x10\x01p}\x04\0\x04blob\x03\0\0\x01r\x03\x04precy\x05scalez\x05values\
\x04\0\x07decimal\x03\0\x02\x01r\x01\x10days-since-epochz\x04\0\x04date\x03\0\x04\
\x01r\x01\x18milliseconds-since-epochx\x04\0\x08datetime\x03\0\x06\x01r\x01\x1an\
anoseconds-since-midnightx\x04\0\x04time\x03\0\x08\x01q\x0d\x04null\0\0\x04blob\x01\
\x01\0\x06boolen\x01\x7f\0\x03str\x01s\0\x05float\x01u\0\x03int\x01x\0\x07decima\
l\x01\x03\0\x04date\x01\x05\0\x08datetime\x01\x07\0\x04time\x01\x09\0\x0cdatetim\
e-iso\x01s\0\x0cduration-iso\x01s\0\x05error\x01s\0\x04\0\x03val\x03\0\x0a\x01o\x02\
s\x0b\x01p\x0c\x01@\0\0\x0d\x04\0\x07get-map\x01\x0e\x03\0\x15local:val/types@1.\
0.0\x05\0\x02\x03\0\0\x03val\x03\0\x03val\x03\0\x01\x01B\x10\x01p}\x04\0\x04blob\
\x03\0\0\x01r\x03\x04precy\x05scalez\x05values\x04\0\x07decimal\x03\0\x02\x01r\x01\
\x10days-since-epochz\x04\0\x04date\x03\0\x04\x01r\x01\x18milliseconds-since-epo\
chx\x04\0\x08datetime\x03\0\x06\x01r\x01\x1ananoseconds-since-midnightx\x04\0\x04\
time\x03\0\x08\x01q\x0d\x04null\0\0\x04blob\x01\x01\0\x06boolen\x01\x7f\0\x03str\
\x01s\0\x05float\x01u\0\x03int\x01x\0\x07decimal\x01\x03\0\x04date\x01\x05\0\x08\
datetime\x01\x07\0\x04time\x01\x09\0\x0cdatetime-iso\x01s\0\x0cduration-iso\x01s\
\0\x05error\x01s\0\x04\0\x03val\x03\0\x0a\x01o\x02s\x0b\x01p\x0c\x01@\0\0\x0d\x04\
\0\x07get-map\x01\x0e\x04\0\x15local:val/types@1.0.0\x05\x03\x04\0\x13local:val/\
val@1.0.0\x04\0\x0b\x09\x01\0\x03val\x03\0\0\0G\x09producers\x01\x0cprocessed-by\
\x02\x0dwit-component\x070.227.1\x10wit-bindgen-rust\x060.41.0";

#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
  wit_bindgen::rt::maybe_link_cabi_realloc();
}

