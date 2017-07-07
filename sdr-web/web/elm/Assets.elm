module Assets exposing (..)


type AssetPath
    = AssetPath String


star =
    AssetPath "../../priv/static/images/eye.svg"


path : AssetPath -> String
path (AssetPath str) =
    str
