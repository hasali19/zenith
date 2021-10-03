use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn copy_stream<I, E>(
    mut input: I,
    mut output: impl AsyncWrite + Unpin,
) -> eyre::Result<()>
where
    I: Stream<Item = Result<bytes::Bytes, E>> + Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    while let Some(chunk) = input.try_next().await? {
        output.write_all(chunk.as_ref()).await?;
    }

    output.flush().await?;

    Ok(())
}

pub fn to_byte_stream(
    input: impl AsyncRead,
) -> impl Stream<Item = Result<bytes::Bytes, std::io::Error>> {
    FramedRead::new(input, BytesCodec::new()).map(|res| res.map(|bytes| bytes.freeze()))
}
