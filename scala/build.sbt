// AtCoderのScalaバージョン
val atCoderScalaVersion = "2.13.1"

ThisBuild / scalafmtOnCompile := true

lazy val root = project
  .in(file("."))
  .settings(
    name := "my scala answers",
    version := "0.1.0",
    scalaVersion := atCoderScalaVersion
  )
  .aggregate(answers, sample)

lazy val answers = project
  .in(file("answers"))
  .settings(
    name := "answers"
  )

lazy val sample = project
  .in(file("sample"))
  .settings(
    name := "sample"
  )
