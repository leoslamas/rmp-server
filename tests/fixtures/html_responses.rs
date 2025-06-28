pub fn l337x_search_page() -> String {
    r#"
    <html>
    <body>
    <table class="table-list table table-responsive table-striped">
    <thead>
    <tr>
    <th class="coll-1 name">name</th>
    <th class="coll-2">se</th>
    <th class="coll-3">le</th>
    <th class="coll-date">time</th>
    <th class="coll-4"><span class="size">size</span> <span class="info">info</span></th>
    <th class="coll-5">uploader</th>
    </tr>
    </thead>
    <tbody>
    <tr>
    <td class="coll-1 name"><a href="/sub/6/0/" class="icon"><i class="flaticon-divx"></i></a><a href="/torrent/Test.Movie.2023/">Test Movie 2023</a></td>
    <td class="coll-2 seeds">1500</td>
    <td class="coll-3 leeches">200</td>
    <td class="coll-date">Dec. 10th '23</td>
    <td class="coll-4 size mob-user">1.5 GB<span class="seeds">1500</span></td>
    <td class="coll-5 user"><a href="/user/TestUploader/">TestUploader</a></td>
    </tr>
    <tr>
    <td class="coll-1 name"><a href="/sub/6/0/" class="icon"><i class="flaticon-divx"></i></a><a href="/torrent/Another.Show.S01E01/">Another Show S01E01</a></td>
    <td class="coll-2 seeds">800</td>
    <td class="coll-3 leeches">50</td>
    <td class="coll-date">Dec. 09th '23</td>
    <td class="coll-4 size mob-user">750 MB<span class="seeds">800</span></td>
    <td class="coll-5 user"><a href="/user/SeriesUploader/">SeriesUploader</a></td>
    </tr>
    </tbody>
    </table>
    </body>
    </html>
    "#.to_string()
}

pub fn l337x_magnet_page() -> String {
    r#"
    <html>
    <body>
    <div class="col-9 page-content">
    <div class="box-info torrent-detail-page  vpn-info-wrap">
    <div class="box-info-heading clearfix"><h1> Test Movie 2023 </h1>
    </div>
    <div class="lf3ba6c418b5f4ee4e1f50b3fbfdced465c96e2d0 no-top-radius">
    <div class="lce207072b5a6a519bafb45db419be47c2d331555 clearfix">
    <ul class="le85aec8b46f88def3aed2f8996f85ac2edd53594 l1b1397bcdc13f88822df0b36abc27cdf17bbd6c5">
    <li><a class="lb6031b0d322cf2f9769768fa16a450c23c954366 l986ff5effa3ea4d6ac47830547ffebda8130f266 ld3a91ea78222b30937f8a1c6b14fcd7e17c8a2a6" href="magnet:?xt=urn:btih:123456789abcdef&dn=Test+Movie+2023" onclick="javascript: count(this);"><span class="icon"><i class="flaticon-l1b18a30c95d2bb796ec169ab55ac2b8c03e90298"></i></span>Magnet Download</a> </li>
    <li style="margin-top:0px;"></li>
    <li class="dropdown">
    </ul>
    </li>
    </body>
    </html>
    "#.to_string()
}