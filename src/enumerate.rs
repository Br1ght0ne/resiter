//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumOkConfig {
    /// Ignore Err(_) when counting
    IgnoreErr,

    /// Increase counter on Err(_)
    CountErr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumErrConfig {
    /// Ignore Ok(_) when counting
    IgnoreOk,

    /// Increase counter on Ok(_)
    CountOk,
}

pub trait EnumerateResult<T, E>
    where Self: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized,
{
    fn enumerate_ok(self, config: EnumOkConfig) -> EnumerateOkImpl<Self, T, E>;
    fn enumerate_err(self, config: EnumErrConfig) -> EnumerateErrImpl<Self, T, E>;
}

impl<I, T, E> EnumerateResult<T, E> for I
    where I: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized
{
    fn enumerate_ok(self, config: EnumOkConfig) -> EnumerateOkImpl<Self, T, E> {
        EnumerateOkImpl(self, 0, config)
    }

    fn enumerate_err(self, config: EnumErrConfig) -> EnumerateErrImpl<Self, T, E> {
        EnumerateErrImpl(self, 0, config)
    }
}


pub struct EnumerateOkImpl<I, T, E>(I, usize, EnumOkConfig)
    where I: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized;

impl<I, T, E> Iterator for EnumerateOkImpl<I, T, E>
    where I: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized,
{
    type Item = Result<(usize, T), E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|o| match o {
            Err(e) => {
                if self.2 == EnumOkConfig::CountErr {
                    self.1 += 1;
                }
                Err(e)
            },

            Ok(t) => {
                self.1 += 1;
                Ok((self.1, t))
            }
        })
    }
}


pub struct EnumerateErrImpl<I, T, E>(I, usize, EnumErrConfig)
    where I: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized;

impl<I, T, E> Iterator for EnumerateErrImpl<I, T, E>
    where I: Iterator<Item = Result<T, E>> + Sized,
          T: Sized,
          E: Sized,
{
    type Item = Result<T, (usize, E)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|o| match o {
            Ok(t) => {
                if self.2 == EnumErrConfig::CountOk {
                    self.1 += 1;
                }
                Ok(t)
            },

            Err(e) => {
                self.1 += 1;
                Err((self.1, e))
            }
        })
    }
}

