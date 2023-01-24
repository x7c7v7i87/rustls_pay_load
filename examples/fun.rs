#![allow(unused_variables)] 
#![allow(unused_must_use)]

pub const CA_FILE: &str = "/Users/q/skill/rf/rustls_load/dev/ca.cert";
pub const CLIENT_CERT_FILE: &str = "/Users/q/skill/rf/rustls_load/dev/client.cert";
pub const CLIENT_KEY_FILE: &str = "/Users/q/skill/rf/rustls_load/dev/client.key";
pub const SERVER_CERT_FILE: &str = "/Users/q/skill/rf/rustls_load/dev/server.fullchain";
pub const SERVER_KEY_FILE: &str = "/Users/q/skill/rf/rustls_load/dev/server.rsa";


pub fn server_key() -> String {String::from(
        "-----BEGIN RSA PRIVATE KEY-----
        MIIEowIBAAKCAQEA0eEF664wSzaZx0NBLxwj8SSCx8g7ggStjJwLeDKnHAAhWLp1
        gb1SfdzK64KWCaEKD3iZ8xudwlhhCpXNZU4q6jFbCKv6+IuG8dyVR8W6CK3trlS+
        GsS61nJQYZ//Xl2Y531VK1PayqtmGnkMyPX0oxA4kNcWoqZu9W74bIwCidbborZg
        KeMz2E2AY4hvbsk6D0E5ka07WuXKmE8+J8Uiyj3m4tegZao0Q/XwJwPBdG885kwa
        cjqL424FCbQbdpVX4vw4cIgCRCc7eikAiX/NrqyM0nEOVRMzPlM75dkYEAlJKH/a
        6ibCrQV/vq1TMAPXRpdkZnUDJAv9DcJF0LGs8QIDAQABAoIBAQCP9OCyv0+wx0rD
        OoZDYfHlPT3W+qiWjc7cW0wirduhgRVeXsaqrskeO4uT7oYrJKJZ6kbuUUrsOuKU
        +BtiGTOmXyQp1ozWXh25V6utBxqLSYF1yfp7MeVVoiMwUyY9+8UdQwJgLawsx1PJ
        ZK44STv7fOzPrDiCZ9ZD0lHjYkV7dP3/UfM79mP7SlOgpNOKpGe52p1juPdeI6fh
        kIGWH/TFjTqixiIkYntDeQRctxJRWRC5hPcDJSd64Y+XwblGSb2XD3cgiBLCQI4E
        gSlSHEOwX4B28cydxyza85fr7xs3ziMQU8QcDHwhICRQUVgX8Eh3Dny812GzQZMA
        OPg4qLYBAoGBAPGpjgLCuv6FbWL/uOOToCqWkwj+U4vZdu2f7sO0SS6lvkHzH76N
        Y6Qvqq5ZGq3KwnyADInTmZy23YxQqJfR3bW7O+pk7/Gl8d0wMd2DEHyYQNZyRNXW
        MN2GQ7ClmAE7d1lsy+8v9cBcehOu922cX8xv8HyoKkvwVEOz7H8Eb0MZAoGBAN5U
        u6UUjE2knoPWNWTcKbr30wGc7V9fBKy9AqcnKbOwlVRjBokr2fNR6XD4t48U5zFf
        bXDF1PLRk1+Q/ZEssiToNlCYvhaqCP3bE+W6i83ZZevqzEwWtol2c8JKMRpA50Di
        QMQjEQZtuVxi3s6beZQRbhIYS1ydQsg9HB5DZIuZAoGADe5aU2pqYTXL4oMfY28l
        Mo9PlpsisdMtefFYUEvaVCgV2bNsRw3biqF802Qrl40yvP6cum1KU7BvASG9NlNQ
        1qMehxqegm2wjbGzjjN6BdS5GCOqODGKy5pr2IkTRC7raGpPL1CyeA5csznI5ba4
        Bw8E7Aq9tXb7RJPgtEnLKDECgYB4AuUHRwA1KRcnswk/WFhuRDfyNvMq9+9eDujP
        saQhCsCQvKNaiAEuBpksEhWDdpUVYZ2BskgwilAy1eEJ7EZ9BZvMBacWNNKFtSwY
        ZYnxo9gQcL22lMwTh40U41d+BGrP61FqjkMyZSgZZoU+oU4PTxlLSzHi3ECgNdxn
        uZkcgQKBgDmNffCsJPBwQItVVauoeLYplWkDlWIdnfnrs0UqhJTZNShNlf9/mGqi
        i++Hv/GSlrZ1Yo8gg+61UoSv9chClvmYmT+NO/lvIHgpg+mVYm9fT0dkVJAIRTg7
        ErL2fS4lYH3AmBH9a1DKNJ6B3Zoh5Mnt1o37l/zz72tTRyCQtObV
        -----END RSA PRIVATE KEY-----",
    )
}
pub fn server_cert() -> String {String::from(
        "-----BEGIN CERTIFICATE-----
        MIIENDCCAhygAwIBAgICAcgwDQYJKoZIhvcNAQELBQAwADAeFw0yMzAxMTgxNTMy
        NDBaFw0yODA3MTAxNTMyNDBaMAAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEK
        AoIBAQDR4QXrrjBLNpnHQ0EvHCPxJILHyDuCBK2MnAt4MqccACFYunWBvVJ93Mrr
        gpYJoQoPeJnzG53CWGEKlc1lTirqMVsIq/r4i4bx3JVHxboIre2uVL4axLrWclBh
        n/9eXZjnfVUrU9rKq2YaeQzI9fSjEDiQ1xaipm71bvhsjAKJ1tuitmAp4zPYTYBj
        iG9uyToPQTmRrTta5cqYTz4nxSLKPebi16BlqjRD9fAnA8F0bzzmTBpyOovjbgUJ
        tBt2lVfi/DhwiAJEJzt6KQCJf82urIzScQ5VEzM+Uzvl2RgQCUkof9rqJsKtBX++
        rVMwA9dGl2RmdQMkC/0NwkXQsazxAgMBAAGjgbcwgbQwDAYDVR0TAQH/BAIwADAL
        BgNVHQ8EBAMCBsAwHQYDVR0OBBYEFNuBPupSliVTbVxhV2J+8NbtDmI8MDsGA1Ud
        IwQ0MDKAFP8pW0t0HgZivlMj24VBBZ0RBsoloQSkAjAAghRRpvGEwEkJG8RkLrBr
        jik3dvq5XTA7BgNVHREENDAygg50ZXN0c2VydmVyLmNvbYIVc2Vjb25kLnRlc3Rz
        ZXJ2ZXIuY29tgglsb2NhbGhvc3QwDQYJKoZIhvcNAQELBQADggIBABmVAWMcJxDw
        gvt4UmIbgS/ZJXPtcap7gAxdRa0Ic0GZ1pwDVrLuztln5Ij9rHVXGwHIOJ5NqHvU
        7zS5Z14Xf3mpaRQQ+5uIK0Ts7Z5uOj2JedW8oW5PGUS34LujlD+x6ba4E1szVEYH
        uIJNa4QqbmFp5IWLNBsJiMmusl038iYmXQFOMuQTLfMfYe/9AMLzkogZPoW5Zcqu
        PfnS+NTaRasU3u0m9BL8SGmiDQ+YfG7SpW7G7WyMmNtzXQbEBsnITtBl51pKIrq3
        Z6uerE/rL983QlLC1EdRMiT8SvK5QhtL2I7sD84rRVvl71fgfB5yVUUbWybyuov/
        ypvxSxzil6ASheVrqlEtuJjYg9uqcgQXUPlCqpBPaVLdS0NLLceT1VnQ1S+Ie5Ax
        HueXbWr3HAyGJcc/i8xo8ibwbaW9ARX72TCyW/7l+kPmtuEzZoCVbMNWBy4ERndi
        ebUFrUg736yHrpX2aVMyxuQmooiKuLsewn6AQh6yEoy4talogYbcg+Wgk9QY/Vjx
        ENYn61qwjW2r3gXzv8O4P1a4dJdcJLJEwLddswSPsEmO2svyrDxf+6BKoGMQXy+4
        9h7mbNNA1tiLe3hTKEiUWqCtFDoJHikCqnsOnE9DHXKNl92+0J41KQaJ6ATiv4gL
        JAF7a2ukee4zIl8rq7RtGJFA2bhn8ywv
        -----END CERTIFICATE-----
        -----BEGIN CERTIFICATE-----
        MIIE4TCCAsmgAwIBAgIUUabxhMBJCRvEZC6wa44pN3b6uV0wDQYJKoZIhvcNAQEL
        BQAwADAeFw0yMzAxMTgxNTMyNDBaFw0zMzAxMTUxNTMyNDBaMAAwggIiMA0GCSqG
        SIb3DQEBAQUAA4ICDwAwggIKAoICAQDmMIQqTlVHcVHW+6f0w8PrordMBRbUvtcS
        N+8idJdacaK2/dQy9x/frdmKAxEEjUgXFRY1DXzMe7yNkODrWcLSIITUyxk/RWgA
        KO0s3prZ+ppMBzdKaSI5NWiEPTaBTUrUzWu9GenHwYFUXa3hAtATh4b+i2JyCZna
        oOr32BuaLS7PY3311sbTLG495nLz0BnJoU63TMMLTYoBS+OKXx3VdqBTY6Vu4E6n
        vMy4U8gQgY4GIR1jhcd0ZTEBqO2OEi5I6oBhGxgazrkFCSUX6OgvoLdq5f85jvkz
        hcdibPXBCmF65zdAg4FgV8yiNtBZkrJKTIhxo/MJFPxSAJyC2YIJW2ROi5ieVPdF
        U30HpObUqo8uCsmXt4x6W6LSdWcncW60WbdHGQXfqpcUJaiZZV6xgNrd5UgpFboN
        KLbCoiRGoxvFWWg3trYp1XdymS/NUJivj4z5Ue9yqHI1/hw2ttnzcwTtBENqaKY+
        ArrichFl8Y/BqOrK+hlmBFuRuWd4b0R3cukvTfgjBbNWN/rx1ejotwStSBq9a0GW
        6TTFSNqqmRRxmG4cHc8IHGoM4HqfkB9kPNCRSfCZOoM1HCeKC9y6NeAi6qYMLYRA
        p7iWnY3TpJbsLVCKZANPJTmZl6nUqpyb6vIYEA6ONvbh+Dh2BX80dJlzAW+cVbyy
        ZftSyGUMtwIDAQABo1MwUTAdBgNVHQ4EFgQU/ylbS3QeBmK+UyPbhUEFnREGyiUw
        HwYDVR0jBBgwFoAU/ylbS3QeBmK+UyPbhUEFnREGyiUwDwYDVR0TAQH/BAUwAwEB
        /zANBgkqhkiG9w0BAQsFAAOCAgEA1jAocRBQ7jL3nAXCFU3nWQVpkH9QxSZNA8XC
        jXdhDmomDMsqniMgAndWgfUCZJHIBNyFs4wBTjl8j3vY0OakJi+DgCa8Ao45l2dc
        Jf7j+b8LHCXSi+K8IZZSgCpornaBYYlJmTyiZeJG5KeyUW65/Y2ISi3k2RqbrrcV
        87GrCFBIMzrDILGJPHk7PUGTZhmN2m/5aQvu/HWI3AasZIO3WhNRDwV0zomP+eh6
        tHrjdDLBcH0BbyiOniIJ6BeALZZY9aT1JqEGfjW9IsO8LYcdckploj2shCj7VGmD
        3PDUL5i6NjzGvoGygFOGj4yTXCu7xNoxB/OTKiQ/P6SDFhWIE+ISxczfKvZmXgQv
        qq6uOhdEhxFkL6DrGwhkK8d0kSNLfqdetMe5RDP530gWP7fM904vh3HAH9X65vkN
        tQVp/eiLR0eaRDqhGryE1k8h5nJ8NwC0HsC4eIyIY9uXr8FdsNL0TVetImDuAIq9
        QKf0v45PwXnmKMalkTqOxxd5l1LMVlpfMAf4QxsDldfawyfe9ChBKQzqoZeWKLxq
        tEu4EMdgUb7CJqnuYXIGTaVDXC+n8Lx4GvrYtXPbqPUsaZMQSqg1Gc0M4BjcO6ug
        BiS/kddOjyEaq7hJ67LyWlxFfDwGScLABCCyRzctGCssnN4JHS/LLFELRdJWw83q
        a0Sk4tY=
        -----END CERTIFICATE-----",
    )
}
pub fn client_key() -> String {"-----BEGIN PRIVATE KEY-----
        MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC/eP5Rp+q/Krub
        qDrTY25u1df7DAm3UnQ0zTsASFnZ2ddgCWm+f7UN2B/1+PDx3++sbZi/ZpapE+Gu
        Dyeh3pMBZFFeG1IKuI4pgUcpT93FtvX6Bn4fZEWxgQpFSCnzb1xOKXwaYQ+whc++
        7HtFDhGwYeceWQWwPlty6TpCCMqD4134iDx3Y1C4SRT+/79h8TdlYkM6IarbtgOj
        0u9ccil6gqYErRTD5UFw31fE+6pQ/hHqs3BMEyb23D/VF0N6+UehU1VFICg4S05a
        tOoNQ/vcoz4Imfg/rz3zgCMdUTqk+8xADv7LkUf85EO87kXBPeD7JhpSOUaDCvu7
        TusHRJu5AgMBAAECggEAOq7kkzY5tjlsonpe/Sa+U+3qciDwYWU+BGjaKm4CS8h+
        QArig+Y0IXgZ45NWlWsi0+ALLq5Mgql7Q5OVfADxYT3TylnpPSvvNj1PCBRYTh6T
        KPwQb6KV0Z9Q/IbvhJKe4b3JMXSKEHoSf9uOtE1pknuPEDgMjEWwVXmxx0dWejEP
        h5dz9Waop+ZVIVUm3kAjc701yK3pyb4EvAxj/leR7QPIs1zEaO8Ezw1k2JR1xr9P
        X3DDDzY4AqQAD36oRmkpuNMR/ol4q0c57PwufsHmq5ej0uhRLFxmH+ZnVuWCnAtF
        WlTpYFm8qC/NQV16uUFlLn0VWp15f80LXZc9M8uAeQKBgQDeMtrVqjRmkQMLoDjt
        wPI6ERudK157Eco1M/OiJUerdMqVhdIp/oQSd3cqsZ/dTeHgjKqqCvmZ/7jnYElG
        tzAjT7i3AnbS1YooRgsCvrtRp8wPRrc1K/gCzvFdA4kJWcussffHzm6x05O5YQZn
        37W1AXW9PsmG1yfqQH8y9r7h7wKBgQDcmZEC0Tp2CZ2bxAveqX88SsJ2KaIYHNF+
        hhXmySevVrstijsLU3holOVPSLgFQQm19XILGqtkHmsrJAq74M0KSxF+tVL066oq
        pAu3gBUFsqg8nf5XcqxDk4lAijwaiVXHvubC47T9k64fb/uYW/x6ltV0QCLXvlxP
        qw6X/ATk1wKBgQC9DqQn/GQNiQEAWVlW4j63rWrDde0z5dUZI5T+t52dgwgD5RMG
        7CnP1JouV2fgUq4H82gauq/Zmj3nkIOIicGJqgFOpegYR9z/m2ku8r5LIex8avzj
        Xv/s08uGCsYEkMS9scdT2I9S+uTMbIOYyj2/PvjBBrVN569sn2gbItsTPQKBgHS6
        NYceGycWU3V2uSkq2w6WKGvJ/+FublPas4AY9IzNAeJJAJEvq8j68JORUP5GudK0
        e5HwnLwvdFmxipR7kUNfEXTQkYLTqIlDoIeKmJUZQfhBSbIQFaL0UG38N217Og6l
        rwl03/JF0J8F7EVkdyaO5YfL4CP0sHffdatqSVSlAoGBAIRIsK3EhJHLyXtDb0n/
        AX+lwYC28eVYr7OEN7Th2l7agWM0jglTTTpAuVEfipcYD5P++heAWKjq3vYrOBVc
        0KAdVeDlivp9WcQuGGQYoawY5Tb/Fyu1aP8wISoQ/IgbntZa5KM00iJp1t77ZvUA
        6Elb2YpntPwKss7cCJ86tljw
        -----END PRIVATE KEY-----"
        .to_string()
}
pub fn client_cert() -> String {"-----BEGIN CERTIFICATE-----
        MIIEDzCCAfegAwIBAgICAxUwDQYJKoZIhvcNAQELBQAwADAeFw0yMzAxMTgxNTMy
        NDBaFw0yODA3MTAxNTMyNDBaMAAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEK
        AoIBAQC/eP5Rp+q/KrubqDrTY25u1df7DAm3UnQ0zTsASFnZ2ddgCWm+f7UN2B/1
        +PDx3++sbZi/ZpapE+GuDyeh3pMBZFFeG1IKuI4pgUcpT93FtvX6Bn4fZEWxgQpF
        SCnzb1xOKXwaYQ+whc++7HtFDhGwYeceWQWwPlty6TpCCMqD4134iDx3Y1C4SRT+
        /79h8TdlYkM6IarbtgOj0u9ccil6gqYErRTD5UFw31fE+6pQ/hHqs3BMEyb23D/V
        F0N6+UehU1VFICg4S05atOoNQ/vcoz4Imfg/rz3zgCMdUTqk+8xADv7LkUf85EO8
        7kXBPeD7JhpSOUaDCvu7TusHRJu5AgMBAAGjgZIwgY8wDAYDVR0TAQH/BAIwADAL
        BgNVHQ8EBAMCBsAwFgYDVR0lAQH/BAwwCgYIKwYBBQUHAwIwHQYDVR0OBBYEFBd/
        56gTqtOrSLz39IndaLgxKKQYMDsGA1UdIwQ0MDKAFP8pW0t0HgZivlMj24VBBZ0R
        BsoloQSkAjAAghRRpvGEwEkJG8RkLrBrjik3dvq5XTANBgkqhkiG9w0BAQsFAAOC
        AgEAmMjm7SglG2WExjot+Up7k3dxH3Lva1u4DI9bs3/FU5hlISnHD1LFPfMRZ/+L
        pipo19DLZs4r0IPbQpvKN0x8pewr1dO8xbfsSy0VQtwtuMXknKXRQqcpvjGnNdU4
        jey7AuccAwfiyelY8Sh0oeDyj5/rCQwMRCD0Dv1ldLSdpo+5UlWV7CnmMHRNmjZx
        KFUxFRD4vXReo/GIKe0JFsX3ah5EZcIEe44WVSzXlPZqlfgkGStob1oLV8JedNpt
        62RZbPVb6dvNG7CsOHP9FmMN5l7UgtejZEp5XZ4OiueKa+ehEBakm2ZTs7dowbyZ
        wtCZfs4M/5zIAgs6qlrX6rqWCERsDetxgdf0Pv5qQKIOYO0CYNvhZJ+g9VCLVrI7
        p227S+EEKD3m+UKCN368si0QiAmQ7MNOVc6MxXlepEGRlHDwhyjgDywCTpgUzPLZ
        0FRbQfZsQaJ0JC8nQaolzFPC5xdMv0NLPWmeZxBTKYux8jj2gS0OoAyCNGw91dhy
        ztS+YMfPbWIYDvqMnZdka1B8pQ3ozNbMwbkg3f2RnSGo+RbxiH3hCB7hlqXKkcdf
        H4EjouEksLogVpfXWBAQGj4tsWrH0UDWh+uxs2AUX6ZHIqSF23ouERUThrNqOCvh
        kdpAqXKlq9tYHQUL8ygT1K9RWVVtWD5/UNeobpwJbct1YKQ=
        -----END CERTIFICATE-----"
        .to_string()
}
pub fn get_ca() -> String {"-----BEGIN CERTIFICATE-----
        MIIEDzCCAfegAwIBAgICAxUwDQYJKoZIhvcNAQELBQAwADAeFw0yMzAxMTgxNTMy
        NDBaFw0yODA3MTAxNTMyNDBaMAAwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEK
        AoIBAQC/eP5Rp+q/KrubqDrTY25u1df7DAm3UnQ0zTsASFnZ2ddgCWm+f7UN2B/1
        +PDx3++sbZi/ZpapE+GuDyeh3pMBZFFeG1IKuI4pgUcpT93FtvX6Bn4fZEWxgQpF
        SCnzb1xOKXwaYQ+whc++7HtFDhGwYeceWQWwPlty6TpCCMqD4134iDx3Y1C4SRT+
        /79h8TdlYkM6IarbtgOj0u9ccil6gqYErRTD5UFw31fE+6pQ/hHqs3BMEyb23D/V
        F0N6+UehU1VFICg4S05atOoNQ/vcoz4Imfg/rz3zgCMdUTqk+8xADv7LkUf85EO8
        7kXBPeD7JhpSOUaDCvu7TusHRJu5AgMBAAGjgZIwgY8wDAYDVR0TAQH/BAIwADAL
        BgNVHQ8EBAMCBsAwFgYDVR0lAQH/BAwwCgYIKwYBBQUHAwIwHQYDVR0OBBYEFBd/
        56gTqtOrSLz39IndaLgxKKQYMDsGA1UdIwQ0MDKAFP8pW0t0HgZivlMj24VBBZ0R
        BsoloQSkAjAAghRRpvGEwEkJG8RkLrBrjik3dvq5XTANBgkqhkiG9w0BAQsFAAOC
        AgEAmMjm7SglG2WExjot+Up7k3dxH3Lva1u4DI9bs3/FU5hlISnHD1LFPfMRZ/+L
        pipo19DLZs4r0IPbQpvKN0x8pewr1dO8xbfsSy0VQtwtuMXknKXRQqcpvjGnNdU4
        jey7AuccAwfiyelY8Sh0oeDyj5/rCQwMRCD0Dv1ldLSdpo+5UlWV7CnmMHRNmjZx
        KFUxFRD4vXReo/GIKe0JFsX3ah5EZcIEe44WVSzXlPZqlfgkGStob1oLV8JedNpt
        62RZbPVb6dvNG7CsOHP9FmMN5l7UgtejZEp5XZ4OiueKa+ehEBakm2ZTs7dowbyZ
        wtCZfs4M/5zIAgs6qlrX6rqWCERsDetxgdf0Pv5qQKIOYO0CYNvhZJ+g9VCLVrI7
        p227S+EEKD3m+UKCN368si0QiAmQ7MNOVc6MxXlepEGRlHDwhyjgDywCTpgUzPLZ
        0FRbQfZsQaJ0JC8nQaolzFPC5xdMv0NLPWmeZxBTKYux8jj2gS0OoAyCNGw91dhy
        ztS+YMfPbWIYDvqMnZdka1B8pQ3ozNbMwbkg3f2RnSGo+RbxiH3hCB7hlqXKkcdf
        H4EjouEksLogVpfXWBAQGj4tsWrH0UDWh+uxs2AUX6ZHIqSF23ouERUThrNqOCvh
        kdpAqXKlq9tYHQUL8ygT1K9RWVVtWD5/UNeobpwJbct1YKQ=
        -----END CERTIFICATE-----"
        .to_string()
}

pub fn main(){
    println!("Hello, world!");
}
