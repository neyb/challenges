tasks.test {
    useJUnitPlatform {
        includeEngines = setOf("junit-platform-suite")
        excludeTags.add("slow")
    }
}