package app.olauncher.helper

object OccasionNative {
    external fun outputOf(configJson: String): String
    external fun defaultConfig(): String
    external fun validate(configJson: String): Boolean

    init {
        System.loadLibrary("occasion_android")
    }
}