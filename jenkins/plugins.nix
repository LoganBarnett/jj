# Jenkins plugin definitions — configuration-as-code, workflow-aggregator,
# extended-choice-parameter and all of their required transitive dependencies.
#
# To regenerate this file (e.g. after adding a plugin or updating versions):
#
#   python3 jenkins/generate-plugins.py > jenkins/plugins.nix
#
# The generator queries the Jenkins stable update center, resolves transitive
# dependencies, fetches each .hpi, and writes this file with correct hashes.
# Adding a new top-level plugin means editing the `wanted` list at the top of
# generate-plugins.py and re-running it.
{
  fetchurl,
  stdenv,
}: {
  "caffeine-api" = stdenv.mkDerivation {
    name = "caffeine-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/caffeine-api/3.2.4-208.v7e2da_a_7db_82b_/caffeine-api.hpi";
      hash = "sha256-QOJUui8DGmWCBE4JUptDyaAxu69lY6BMMnPgRVU6qvE=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "commons-lang3-api" = stdenv.mkDerivation {
    name = "commons-lang3-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/commons-lang3-api/3.20.0-109.ve43756e2d2b_4/commons-lang3-api.hpi";
      hash = "sha256-qgkI4AAsI8Mp3x9eNtLqmvbfvXx/cfz0E8xHHzNcIFI=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "commons-text-api" = stdenv.mkDerivation {
    name = "commons-text-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/commons-text-api/1.15.0-218.va_61573470393/commons-text-api.hpi";
      hash = "sha256-riq9+VdpT0qjm5ugtTntq8OE89ondVGHnjT/si+xN2Q=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "json-api" = stdenv.mkDerivation {
    name = "json-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/json-api/20260719-223.va_81f828cdb_58/json-api.hpi";
      hash = "sha256-95aT4/IKxdRpUoQGdahVDW54Kngb8tn0SmE7Z4TwlsQ=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "structs" = stdenv.mkDerivation {
    name = "structs";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/structs/362.va_b_695ef4fdf9/structs.hpi";
      hash = "sha256-oKXbRPLLCCqexlcsCINAUh8DP/+MNR/H66nvfwfbv40=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-step-api" = stdenv.mkDerivation {
    name = "workflow-step-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-step-api/724.v538c2362b_dfb_/workflow-step-api.hpi";
      hash = "sha256-iHSR3kRBdCKEcZk7Cy25WNsuf7V3hXyFFsWOlhgEmJY=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "asm-api" = stdenv.mkDerivation {
    name = "asm-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/asm-api/9.10.1-216.va_9256d3b_844b_/asm-api.hpi";
      hash = "sha256-KSyQ09//LpJC+AlnB6P6PZeMtXpramzmicadwho4G+Y=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "scm-api" = stdenv.mkDerivation {
    name = "scm-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/scm-api/728.vc30dcf7a_0df5/scm-api.hpi";
      hash = "sha256-EzZHKYfaovqRIhCroA4eN7c5zgP3XJc6Z5TRoPVcAyY=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-api" = stdenv.mkDerivation {
    name = "workflow-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-api/1413.v2ff1a_5e720fa_/workflow-api.hpi";
      hash = "sha256-9K29azSioLZDoSuJgRC9l2NQFC7/YyLv2iHvrLzLaDU=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "script-security" = stdenv.mkDerivation {
    name = "script-security";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/script-security/1402.1405.vc96e74964250/script-security.hpi";
      hash = "sha256-wa5ns1DtDXmpUIgAFY53G0UYpK0t2MsHYjLy0r2oMo0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-support" = stdenv.mkDerivation {
    name = "workflow-support";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-support/1015.v785e5a_b_b_8b_22/workflow-support.hpi";
      hash = "sha256-kU7nF6uMHWDQ0+pJoa8R5OplwedMC0TcNSL5ov5bKo4=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "plugin-util-api" = stdenv.mkDerivation {
    name = "plugin-util-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/plugin-util-api/7.1341.v039f146993d9/plugin-util-api.hpi";
      hash = "sha256-btkeS3PK5VMntBq5GbU8M7OUJXJxFdbDCPRa56bkeDc=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "font-awesome-api" = stdenv.mkDerivation {
    name = "font-awesome-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/font-awesome-api/7.3.1-1013.v0835a_879ec6d/font-awesome-api.hpi";
      hash = "sha256-fuOCxnhUX1/VsCLl27ddEpqX2ysBgPtC8X7c69/bv1o=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "bootstrap5-api" = stdenv.mkDerivation {
    name = "bootstrap5-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/bootstrap5-api/5.3.8-1048.va_c299057e35c/bootstrap5-api.hpi";
      hash = "sha256-QCe0zsvbQAxM32vu01HLJ6Lg70w50nYY40L612pEkN8=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "antisamy-markup-formatter" = stdenv.mkDerivation {
    name = "antisamy-markup-formatter";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/antisamy-markup-formatter/173.v680e3a_b_69ff3/antisamy-markup-formatter.hpi";
      hash = "sha256-KbR2V5fybERXSicy3fnX2Y6gKDKVNSSX3Z7vhK7QEC8=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "prism-api" = stdenv.mkDerivation {
    name = "prism-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/prism-api/1.30.0-741.v034eb_0b_0a_a_fa_/prism-api.hpi";
      hash = "sha256-K3qngJ4PZWr86YwLagqbYoha9q6FrZC+1npPGPGOg1Q=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "snakeyaml-api" = stdenv.mkDerivation {
    name = "snakeyaml-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/snakeyaml-api/2.5-149.v72471e9c6371/snakeyaml-api.hpi";
      hash = "sha256-gO2+7BW5r7yhjQAM82fZ4mRWqoMQDc3QnuDtakuYqcI=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "configuration-as-code" = stdenv.mkDerivation {
    name = "configuration-as-code";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/configuration-as-code/2100.vb_fd699d2a_09c/configuration-as-code.hpi";
      hash = "sha256-3wTcDKvXBwiqCIpCdPUQOg75aDwnkXvjSSbEge0KOL4=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "ionicons-api" = stdenv.mkDerivation {
    name = "ionicons-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/ionicons-api/94.vcc3065403257/ionicons-api.hpi";
      hash = "sha256-lfNQQ3XWKNiHQlzN8y1WBqIwJh7kLcaD8TlY2OlzrD8=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-scm-step" = stdenv.mkDerivation {
    name = "workflow-scm-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-scm-step/466.va_d69e602552b_/workflow-scm-step.hpi";
      hash = "sha256-jzW2gVPuimcdAdzcPYbdR4N70tX8+d+NZ0/4ruz7oWo=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-cps" = stdenv.mkDerivation {
    name = "workflow-cps";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-cps/4350.vcc65d4958821/workflow-cps.hpi";
      hash = "sha256-hVOHrC/ItVEYdrG9c0a+bYjPWvkpN1OfUevhB7I6IME=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "cloudbees-folder" = stdenv.mkDerivation {
    name = "cloudbees-folder";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/cloudbees-folder/6.1100.ve9eed61d16c4/cloudbees-folder.hpi";
      hash = "sha256-eW8s5RafCclDdD8FZhZ1np9RmI1rgxQOxLNTzI1q86A=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "variant" = stdenv.mkDerivation {
    name = "variant";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/variant/70.va_d9f17f859e0/variant.hpi";
      hash = "sha256-EuIU6mlEaaRGG1WIHstAdK5d+wR5fbwNrTkPb0vHWq8=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-groovy-lib" = stdenv.mkDerivation {
    name = "pipeline-groovy-lib";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-groovy-lib/798.v5cc688825312/pipeline-groovy-lib.hpi";
      hash = "sha256-QCT8FMtTe7D+99/ebpDZv42R0YrvVGfkTztBz7F5Ng4=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jakarta-activation-api" = stdenv.mkDerivation {
    name = "jakarta-activation-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-activation-api/2.1.4-1/jakarta-activation-api.hpi";
      hash = "sha256-5iKYNBdC8Us3QEU4e6ZBrYoZN2eTx2va/rtKBbUJlxI=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jakarta-mail-api" = stdenv.mkDerivation {
    name = "jakarta-mail-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-mail-api/2.1.5-1/jakarta-mail-api.hpi";
      hash = "sha256-JEc16oWlEzLAedI1pXE+cNOBC+piMyiUACrMOZFM4Rs=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "durable-task" = stdenv.mkDerivation {
    name = "durable-task";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/durable-task/686.v80ff80875b_82/durable-task.hpi";
      hash = "sha256-H1u5LuV4TuAaTk4DB+gG+h4mDbis66Vv6NJ36Tknjfo=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-durable-task-step" = stdenv.mkDerivation {
    name = "workflow-durable-task-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-durable-task-step/1479.v56e587f413a_7/workflow-durable-task-step.hpi";
      hash = "sha256-oPDxRkzjWS920PAHnOn8LUJyWU+ZW/PRp+3kzVAxRS4=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "apache-httpcomponents-client-4-api" = stdenv.mkDerivation {
    name = "apache-httpcomponents-client-4-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/apache-httpcomponents-client-4-api/4.5.14-269.vfa_2321039a_83/apache-httpcomponents-client-4-api.hpi";
      hash = "sha256-zqL2HMcokLli/Ut2fDF3jGyzABW0qHWLTCa2LlHKlTM=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "bouncycastle-api" = stdenv.mkDerivation {
    name = "bouncycastle-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/bouncycastle-api/2.30.1.84-291.v9f17b_21896e2/bouncycastle-api.hpi";
      hash = "sha256-oxCZxU2bGsb4ixM2CIpzEkgxUyMTvzKA4+sWEDADguE=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "instance-identity" = stdenv.mkDerivation {
    name = "instance-identity";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/instance-identity/203.v15e81a_1b_7a_38/instance-identity.hpi";
      hash = "sha256-3ysyBaYXckjxKylGUKYEBfZbos1QPKlZmy3LcsHeUBg=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "display-url-api" = stdenv.mkDerivation {
    name = "display-url-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/display-url-api/2.217.va_6b_de84cc74b_/display-url-api.hpi";
      hash = "sha256-B+6q5zi0UbreR1aQC0hWHGxJOkLHugekWyKi3OcawMo=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "mailer" = stdenv.mkDerivation {
    name = "mailer";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/mailer/534.v1b_36f5864073/mailer.hpi";
      hash = "sha256-VCamaj+HnSX8tLr/Z4ooQir7q/8izW5PD5mPxno9w/o=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-basic-steps" = stdenv.mkDerivation {
    name = "workflow-basic-steps";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-basic-steps/1098.v808b_fd7f8cf4/workflow-basic-steps.hpi";
      hash = "sha256-k0SetJRljQZdCX6MVPcHyYyc4+VNovuYKBsKBIcVTq8=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-job" = stdenv.mkDerivation {
    name = "workflow-job";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-job/1590.v49101d088542/workflow-job.hpi";
      hash = "sha256-KZP4thNhTCtXfdgC5ySwcl+8ZP/Jpb/nNwesMTrHtGE=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "branch-api" = stdenv.mkDerivation {
    name = "branch-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/branch-api/2.1280.v0d4e5b_b_460ef/branch-api.hpi";
      hash = "sha256-+pwkZkThtQESQaLm2t7QWsUOWfa5lBhxoBKznv6cjes=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-multibranch" = stdenv.mkDerivation {
    name = "workflow-multibranch";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-multibranch/821.vc3b_4ea_780798/workflow-multibranch.hpi";
      hash = "sha256-8KmFfgsHxSzFTkJ7btyidGEuERyrlBbS48PXaoPwDP0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-build-step" = stdenv.mkDerivation {
    name = "pipeline-build-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-build-step/599.v4b_67ea_11b_152/pipeline-build-step.hpi";
      hash = "sha256-eq71Jq8gufA05cLGSc5FT9rVX19ruRfh4sU8KSg64zY=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "credentials" = stdenv.mkDerivation {
    name = "credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/credentials/1506.v948b_b_b_7dec44/credentials.hpi";
      hash = "sha256-+5MJE4hyw/fl5PAjOzuRwP6NArAeFtAzB9LIb1XQ7A0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-input-step" = stdenv.mkDerivation {
    name = "pipeline-input-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-input-step/560.v56198a_642157/pipeline-input-step.hpi";
      hash = "sha256-fGG7dF7NHk5ceGTcfINSFI2bIoG8ylpI/Tp8ynKQoTU=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-milestone-step" = stdenv.mkDerivation {
    name = "pipeline-milestone-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-milestone-step/152.v6e22b_8cfc66c/pipeline-milestone-step.hpi";
      hash = "sha256-yzGHgJ4hnwG0IGjydwWKgYAz88/RTafsWchvAo/Dg4Y=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-stage-step" = stdenv.mkDerivation {
    name = "pipeline-stage-step";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-stage-step/345.va_96187909426/pipeline-stage-step.hpi";
      hash = "sha256-LrpDOyjy38kFGeIzsUJkiQ+XuTt44lxmTVZgcUH82Us=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "plain-credentials" = stdenv.mkDerivation {
    name = "plain-credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/plain-credentials/199.v9f8e1f741799/plain-credentials.hpi";
      hash = "sha256-teXjXPk607gYqYORbLzfP3hmaZQQpN+SPmopaOzDpEg=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "ssh-credentials" = stdenv.mkDerivation {
    name = "ssh-credentials";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/ssh-credentials/372.va_250881b_08cd/ssh-credentials.hpi";
      hash = "sha256-e00DWwWzMQOPPfqZ3f7wyrus7ejwrSpsqWEfsIRBCKk=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "credentials-binding" = stdenv.mkDerivation {
    name = "credentials-binding";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/credentials-binding/728.v902a_273b_8947/credentials-binding.hpi";
      hash = "sha256-vX0aGH41yOK2YZzVDy+MXSiq/E74/Z3lcvss+kZPRm0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "joda-time-api" = stdenv.mkDerivation {
    name = "joda-time-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/joda-time-api/2.14.2-193.v422b_efce56e0/joda-time-api.hpi";
      hash = "sha256-mgBfoavZlIKfyJXL4pQCzjwsXC+EdGPgQUm300ftxMQ=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jackson-annotations2-api" = stdenv.mkDerivation {
    name = "jackson-annotations2-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jackson-annotations2-api/2.22-19.v10a_a_582ea_26e/jackson-annotations2-api.hpi";
      hash = "sha256-8T0KF69vjIk5pu3TCVIAa3fcpHYLH0DgKJw2ORi30X0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jakarta-xml-bind-api" = stdenv.mkDerivation {
    name = "jakarta-xml-bind-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jakarta-xml-bind-api/4.0.9-19.v2b_a_5b_44d9a_1c/jakarta-xml-bind-api.hpi";
      hash = "sha256-L1kieTZofsAPfqYrVQ56bpyGzaNCPzVn/+ZrvfI1/7U=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "javax-activation-api" = stdenv.mkDerivation {
    name = "javax-activation-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/javax-activation-api/1.2.0-8/javax-activation-api.hpi";
      hash = "sha256-6W6IxS7fB7oA+0WybMQRp6lfo9RJGqoaXTZjdmWIBWA=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jaxb" = stdenv.mkDerivation {
    name = "jaxb";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jaxb/2.3.9-143.v5979df3304e6/jaxb.hpi";
      hash = "sha256-ON30z122GlkQRBMML3zLK3qpq+vI/NimlT+NpVDQAu0=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "woodstox-core-api" = stdenv.mkDerivation {
    name = "woodstox-core-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/woodstox-core-api/7.2.1-6.v3718a_a_11f5c4/woodstox-core-api.hpi";
      hash = "sha256-RDuw5lKQHby/Gmuo6Dk1GZeJa9t7LBbTa9wW9f6jPD4=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "jackson2-api" = stdenv.mkDerivation {
    name = "jackson2-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/jackson2-api/2.22.0-439.vb_b_43658d6176/jackson2-api.hpi";
      hash = "sha256-hhmN3jlKfIi8tG8bAXUowZsuYWpNfXf5lSDbsxsGdHc=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-model-api" = stdenv.mkDerivation {
    name = "pipeline-model-api";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-api/2.2291.v2934911987b_6/pipeline-model-api.hpi";
      hash = "sha256-6WrEyVX8oPqsY1k6eetI6ZvRBv3cPfbLJ80cIirUKfM=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-model-extensions" = stdenv.mkDerivation {
    name = "pipeline-model-extensions";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-extensions/2.2291.v2934911987b_6/pipeline-model-extensions.hpi";
      hash = "sha256-eOHkAnNp8AWYcYr8XwA9UZqpFEN2kFfsTkUseDMOPww=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-stage-tags-metadata" = stdenv.mkDerivation {
    name = "pipeline-stage-tags-metadata";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-stage-tags-metadata/2.2291.v2934911987b_6/pipeline-stage-tags-metadata.hpi";
      hash = "sha256-dDVpQr2a48lIxCX5auvfTx25iSFj9KBjh3T8PdGTb3A=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "pipeline-model-definition" = stdenv.mkDerivation {
    name = "pipeline-model-definition";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/pipeline-model-definition/2.2291.v2934911987b_6/pipeline-model-definition.hpi";
      hash = "sha256-oRUG1K2q4NZgMxBgHMTSsdS8jd11dEhtSmPDMNc2uHM=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "workflow-aggregator" = stdenv.mkDerivation {
    name = "workflow-aggregator";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/workflow-aggregator/608.v67378e9d3db_1/workflow-aggregator.hpi";
      hash = "sha256-e5QifBr6AbsmLBSRmtSjM0EvG1HZR8DyGQnsLa/zV9w=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
  "extended-choice-parameter" = stdenv.mkDerivation {
    name = "extended-choice-parameter";
    src = fetchurl {
      url = "https://updates.jenkins.io/download/plugins/extended-choice-parameter/388.ve7b_d0b_920e10/extended-choice-parameter.hpi";
      hash = "sha256-KaAsq+uQwcbXlztg28ZBzysQoxmgVyWRZcioLIZP5Cg=";
    };
    phases = ["installPhase"];
    installPhase = "cp $src $out";
  };
}
