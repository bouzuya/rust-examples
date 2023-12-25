# Google Drive API メモ

- <https://developers.google.com/drive/api/guides/about-sdk?hl=ja>
- ファイルのダウンロード
  <https://developers.google.com/drive/api/guides/manage-downloads?hl=ja>
  - [npm:google-auth-library] and [npm:googleapis]
- エラーハンドリング
  <https://developers.google.com/drive/api/guides/handle-errors?hl=ja>
- リファレンス
  <https://developers.google.com/drive/api/reference/rest/v3?hl=ja>
  - <https://developers.google.com/drive/api/reference/rest/v3/files/get?hl=ja>

```typescript
/**
 * Downloads a file
 * @param{string} realFileId file ID
 * @return{obj} file status
 * */
async function downloadFile(realFileId) {
  // Get credentials and build service
  // TODO (developer) - Use appropriate auth mechanism for your app

  const {GoogleAuth} = require('google-auth-library');
  const {google} = require('googleapis');

  const auth = new GoogleAuth({
    scopes: 'https://www.googleapis.com/auth/drive',
  });
  const service = google.drive({version: 'v3', auth});

  fileId = realFileId;
  try {
    const file = await service.files.get({
      fileId: fileId,
      alt: 'media',
    });
    console.log(file.status);
    return file.status;
  } catch (err) {
    // TODO(developer) - Handle error
    throw err;
  }
}
```

[npm:google-auth-library]: https://www.npmjs.com/package/google-auth-library
[npm:googleapis]: https://www.npmjs.com/package/googleapis
